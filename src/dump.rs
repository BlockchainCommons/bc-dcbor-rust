use std::str::from_utf8;

use crate::{CBOR, known_tags::KnownTags, CBOREncodable, hex::data_to_hex, varint::{EncodeVarInt, MajorType}, string_util::{sanitized, flanked}};

/// Affordances for viewing the encoded binary representation of CBOR as hexadecimal.
impl CBOR {
    /// Returns the encoded hexadecimal representation of this CBOR.
    pub fn hex(&self) -> String {
        data_to_hex(self.cbor_data())
    }

    /// Returns the encoded hexadecimal representation of this CBOR.
    ///
    /// Optionally annotates the output, e.g. breaking the output up into
    /// semantically meaningful lines, formatting dates, and adding names of
    /// known tags.
    pub fn hex_opt(&self, annotate: bool, known_tags: Option<&dyn KnownTags>) -> String {
        if !annotate {
            return self.hex()
        }
        let items = self.dump_items(0, known_tags);
        let note_column = items.iter().fold(0, |largest, item| {
            largest.max(item.format_first_column().len())
        });
        let lines: Vec<_> = items.iter().map(|x| x.format(note_column)).collect();
        lines.join("\n")
    }

    fn dump_items(&self, level: usize, known_tags: Option<&dyn KnownTags>) -> Vec<DumpItem> {
        match self {
            CBOR::Unsigned(n) => vec!(DumpItem::new(level, vec!(self.cbor_data()), Some(format!("unsigned({})", n)))),
            CBOR::Negative(n) => vec!(DumpItem::new(level, vec!(self.cbor_data()), Some(format!("negative({})", n)))),
            CBOR::Bytes(d) => {
                let mut items = vec![
                    DumpItem::new(level, vec!(d.len().encode_varint(MajorType::Bytes)), Some(format!("bytes({})", d.len())))
                ];
                if !d.is_empty() {
                    let mut note: Option<String> = None;
                    if let Some(a) = from_utf8(d.data()).ok() {
                        if let Some(b) = sanitized(a) {
                            note = Some(flanked(&b, "\"", "\""));
                        }
                    }
                    items.push(DumpItem::new(level + 1, vec!(d.data().to_owned()), note));
                }
                items
            },
            CBOR::Text(s) => {
                let header = s.len().encode_varint(MajorType::Text);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                let utf8_data = s.as_bytes().to_vec();
                vec![
                    DumpItem::new(level, header_data, Some(format!("text({})", utf8_data.len()))),
                    DumpItem::new(level + 1, vec![utf8_data], Some(flanked(s, "\"", "\"")))
                ]
            },
            CBOR::Simple(v) => {
                let data = v.cbor_data();
                let note = format!("{}", v);
                vec![
                    DumpItem::new(level, vec![data], Some(note))
                ]
            },
            CBOR::Tagged(tag, item) => {
                let header = tag.value().encode_varint(MajorType::Tagged);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                let mut note_components: Vec<String> = vec![format!("tag({})", tag.value())];
                if let Some(known_tags) = known_tags {
                    if let Some(name) = known_tags.assigned_name_for_tag(tag) {
                        note_components.push(format!("  ; {}", name));
                    }
                }
                let tag_note = note_components.join(" ");
                vec![
                    vec![
                        DumpItem::new(level, header_data, Some(tag_note))
                    ],
                    item.dump_items(level + 1, known_tags)
                ].into_iter().flat_map(|x| x).collect()
            },
            CBOR::Array(array) => {
                let header = array.len().encode_varint(MajorType::Array);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                vec![
                    vec![
                        DumpItem::new(level, header_data, Some(format!("array({})", array.len())))
                    ],
                    array.iter().flat_map(|x| x.dump_items(level + 1, known_tags)).collect()
                ].into_iter().flat_map(|x| x).collect()
            },
            CBOR::Map(m) => {
                let header = m.len().encode_varint(MajorType::Map);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                vec![
                    vec![
                        DumpItem::new(level, header_data, Some(format!("map({})", m.len())))
                    ],
                    m.iter().flat_map(|x| {
                        vec![
                            x.0.dump_items(level + 1, known_tags),
                            x.1.dump_items(level + 1, known_tags)
                        ].into_iter().flat_map(|x| x).collect::<Vec<DumpItem>>()
                    }).collect()
                ].into_iter().flat_map(|x| x).collect()
            },
        }
    }
}

struct DumpItem {
    level: usize,
    data: Vec<Vec<u8>>,
    note: Option<String>,
}

impl DumpItem {
    fn new(level: usize, data: Vec<Vec<u8>>, note: Option<String>) -> DumpItem {
        DumpItem { level, data, note }
    }

    fn format(&self, note_column: usize) -> String {
        let column_1 = self.format_first_column();
        let (column_2, padding) = {
            if let Some(note) = &self.note {
                let padding_count = 1.max(40.min(note_column as i64) - (column_1.len() as i64) + 1);
                let padding = " ".repeat(padding_count.try_into().unwrap());
                let column_2 = format!("# {}", note);
                (column_2, padding)
            } else {
                ("".to_string(), "".to_string())
            }
        };
        column_1 + &padding + &column_2
    }

    fn format_first_column(&self) -> String {
        let indent = " ".repeat(self.level * 3);
        let hex: Vec<_> = self.data.iter()
            .map(|x| data_to_hex(x))
            .filter(|x| !x.is_empty())
            .collect();
        let hex = hex.join(" ");
        indent + &hex
    }
}
