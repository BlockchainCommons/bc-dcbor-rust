import_stdlib!();

use crate::{tags_store::TagsStoreTrait, with_tags, CBORCase, TagsStoreOpt, CBOR};

use super::{string_util::{sanitized, flanked}, varint::{EncodeVarInt, MajorType}};

#[derive(Clone, Default)]
pub struct HexFormatOpts<'a> {
    annotate: bool,
    tags: TagsStoreOpt<'a>,
}

impl<'a> HexFormatOpts<'a> {
    /// Sets whether to annotate the hex dump with tags.
    pub fn annotate(mut self, annotate: bool) -> Self {
        self.annotate = annotate;
        self
    }

    /// Sets the formatting context for the hex dump.
    pub fn context(mut self, tags: TagsStoreOpt<'a>) -> Self {
        self.tags = tags;
        self
    }
}

/// Affordances for viewing the encoded binary representation of CBOR as hexadecimal.
impl CBOR {
    /// Returns the encoded hexadecimal representation of this CBOR.
    pub fn hex(&self) -> String {
        hex::encode(self.to_cbor_data())
    }

    /// Returns the encoded hexadecimal representation of this CBOR.
    ///
    /// Optionally annotates the output, e.g. breaking the output up into
    /// semantically meaningful lines, formatting dates, and adding names of
    /// known tags.
    pub fn hex_opt<'a>(&self, opts: HexFormatOpts<'a>) -> String {
        if !opts.annotate {
            return self.hex()
        }
        let items = self.dump_items(0, opts);
        let note_column = items.iter().fold(0, |largest, item| {
            largest.max(item.format_first_column().len())
        });
        // Round up to nearest multiple of 4
        let note_column = ((note_column + 4) & !3) - 1;
        let lines: Vec<_> = items.iter().map(|x| x.format(note_column)).collect();
        lines.join("\n")
    }

    /// Returns the encoded hexadecimal representation of this CBOR, with annotations.
    pub fn hex_annotated(&self) -> String {
        self.hex_opt(HexFormatOpts::default().annotate(true))
    }

    fn dump_items<'a>(&self, level: usize, opts: HexFormatOpts<'a>) -> Vec<DumpItem> {
        match self.as_case() {
            CBORCase::Unsigned(n) => vec!(DumpItem::new(level, vec!(self.to_cbor_data()), Some(format!("unsigned({})", n)))),
            CBORCase::Negative(n) => vec!(DumpItem::new(level, vec!(self.to_cbor_data()), Some(format!("negative({})", -1 - (*n as i128))))),
            CBORCase::ByteString(d) => {
                let mut items = vec![
                    DumpItem::new(level, vec!(d.len().encode_varint(MajorType::ByteString)), Some(format!("bytes({})", d.len())))
                ];
                if !d.is_empty() {
                    let mut note: Option<String> = None;
                    if let Ok(a) = str::from_utf8(d) {
                        if let Some(b) = sanitized(a) {
                            note = Some(flanked(&b, "\"", "\""));
                        }
                    }
                    items.push(DumpItem::new(level + 1, vec!(d.to_vec()), note));
                }
                items
            },
            CBORCase::Text(s) => {
                let header = s.len().encode_varint(MajorType::Text);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                let utf8_data = s.as_bytes().to_vec();
                vec![
                    DumpItem::new(level, header_data, Some(format!("text({})", utf8_data.len()))),
                    DumpItem::new(level + 1, vec![utf8_data], Some(flanked(s, "\"", "\"")))
                ]
            },
            CBORCase::Simple(v) => {
                let data = v.cbor_data();
                let note = format!("{}", v);
                vec![
                    DumpItem::new(level, vec![data], Some(note))
                ]
            },
            CBORCase::Tagged(tag, item) => {
                let header = tag.value().encode_varint(MajorType::Tagged);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                let mut note_components: Vec<String> = vec![format!("tag({})", tag.value())];
                match opts.tags {
                    TagsStoreOpt::None => {},
                    TagsStoreOpt::Global => with_tags!(|tags_store: &dyn TagsStoreTrait| {
                        if let Some(name) = tags_store.assigned_name_for_tag(tag) {
                            note_components.push(name);
                        }
                    }),
                    TagsStoreOpt::Custom(tags_store_trait) => {
                        if let Some(name) = tags_store_trait.assigned_name_for_tag(tag) {
                            note_components.push(name);
                        }
                    },
                }
                let tag_note = note_components.join(" ");
                vec![
                    vec![
                        DumpItem::new(level, header_data, Some(tag_note))
                    ],
                    item.dump_items(level + 1, opts)
                ].into_iter().flatten().collect()
            },
            CBORCase::Array(array) => {
                let header = array.len().encode_varint(MajorType::Array);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                vec![
                    vec![
                        DumpItem::new(level, header_data, Some(format!("array({})", array.len())))
                    ],
                    array.iter().flat_map(|x| x.dump_items(level + 1, opts.clone())).collect()
                ].into_iter().flatten().collect()
            },
            CBORCase::Map(m) => {
                let header = m.len().encode_varint(MajorType::Map);
                let header_data = vec![vec!(header[0]), header[1..].to_vec()];
                vec![
                    vec![
                        DumpItem::new(level, header_data, Some(format!("map({})", m.len())))
                    ],
                    m.iter().flat_map(|x| {
                        vec![
                            x.0.dump_items(level + 1, opts.clone()),
                            x.1.dump_items(level + 1, opts.clone())
                        ].into_iter().flatten().collect::<Vec<DumpItem>>()
                    }).collect()
                ].into_iter().flatten().collect()
            },
        }
    }
}

#[derive(Debug)]
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
                let padding_count = 1.max(39.min(note_column as i64) - (column_1.len() as i64) + 1);
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
        let indent = " ".repeat(self.level * 4);
        let hex: Vec<_> = self.data.iter()
            .map(hex::encode)
            .filter(|x| !x.is_empty())
            .collect();
        let hex = hex.join(" ");
        indent + &hex
    }
}
