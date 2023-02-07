use chrono::{Utc, TimeZone};

use crate::{CBOR, known_tags::KnownTags};

impl CBOR {
    pub fn diagnostic_annotated(&self, annotate: bool, known_tags: &Option<Box<dyn KnownTags>>) -> String {
        self.diag_item(annotate, known_tags).format(annotate)
    }

    pub fn diagnostic(&self) -> String {
        self.diagnostic_annotated(false, &None)
    }

    fn diag_item(&self, annotate: bool, known_tags: &Option<Box<dyn KnownTags>>) -> DiagItem {
        match self {
            CBOR::Unsigned(_) | CBOR::Negative(_) | CBOR::Bytes(_) |
            CBOR::Text(_) | CBOR::Simple(_) => DiagItem::Item(format!("{}", self)),

            CBOR::Array(a) => {
                let begin = "[".to_string();
                let end = "]".to_string();
                let items = a.into_iter().map(|x| x.diag_item(annotate, known_tags)).collect();
                let is_pairs = false;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            },
            CBOR::Map(m) => {
                let begin = "{".to_string();
                let end = "}".to_string();
                let items = m.iter().map(|(key, value)| vec![
                    key.diag_item(annotate, known_tags),
                    value.diag_item(annotate, known_tags)
                ]).flat_map(|x| x).collect();
                let is_pairs = false;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            },
            CBOR::Tagged(tag, item) => {
                let diag_item: DiagItem;
                if annotate && tag.value() == 1 {
                    match **item {
                        CBOR::Unsigned(secs_after_epoch) => {
                            let ts = Utc.timestamp_opt(secs_after_epoch as i64, 0).unwrap().to_rfc3339();
                            diag_item = DiagItem::Item(ts);
                        },
                        CBOR::Negative(secs_before_epoch) => {
                            let ts = Utc.timestamp_opt(secs_before_epoch, 0).unwrap().to_rfc3339();
                            diag_item = DiagItem::Item(ts);
                        },
                        _ => {
                            diag_item = item.diag_item(annotate, known_tags);
                        },
                    }
                } else {
                    diag_item = item.diag_item(annotate, known_tags);
                }
                let begin = tag.value().to_string() + "(";
                let end = ")".to_string();
                let items = vec![diag_item];
                let is_pairs = false;
                let comment = known_tags.as_ref().and_then(|x| x.assigned_name_for_tag(tag));
                DiagItem::Group(begin, end, items, is_pairs, comment)
            },
        }
    }
}

enum DiagItem {
    Item(String),
    Group(String, String, Vec<DiagItem>, bool, Option<String>),
}

impl DiagItem {
    fn format(&self, annotate: bool) -> String {
        self.format2(0, "", annotate)
    }

    fn format2(&self, level: i32, separator: &str, annotate: bool) -> String {
        todo!()
    }
}
