use crate::{CBOR, known_tags::KnownTags, string_util::flanked, Date};

/// Affordances for viewing CBOR in diagnostic notation.
impl CBOR {
    /// Returns a representation of this CBOR in diagnostic notation.
    ///
    /// Optionally annotates the output, e.g. formatting dates and adding names
    /// of known tags.
    pub fn diagnostic_opt(&self, annotate: bool, known_tags: Option<&dyn KnownTags>) -> String {
        self.diag_item(annotate, known_tags).format(annotate)
    }

    /// Returns a representation of this CBOR in diagnostic notation.
    pub fn diagnostic(&self) -> String {
        self.diagnostic_opt(false, None)
    }

    fn diag_item(&self, annotate: bool, known_tags: Option<&dyn KnownTags>) -> DiagItem {
        match self {
            CBOR::Unsigned(_) | CBOR::Negative(_) | CBOR::ByteString(_) |
            CBOR::Text(_) | CBOR::Simple(_) => DiagItem::Item(format!("{}", self)),

            CBOR::Array(a) => {
                let begin = "[".to_string();
                let end = "]".to_string();
                let items = a.iter().map(|x| x.diag_item(annotate, known_tags)).collect();
                let is_pairs = false;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            },
            CBOR::Map(m) => {
                let begin = "{".to_string();
                let end = "}".to_string();
                let items = m.iter().flat_map(|(key, value)| vec![
                    key.diag_item(annotate, known_tags),
                    value.diag_item(annotate, known_tags)
                ]).collect();
                let is_pairs = true;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            },
            CBOR::Tagged(tag, item) => {
                let diag_item: DiagItem;
                if annotate && tag.value() == 1 {
                    match **item {
                        CBOR::Unsigned(secs_after_epoch) => {
                            let date = Date::from_timestamp(secs_after_epoch as i64).to_string();
                            diag_item = DiagItem::Item(date);
                        },
                        CBOR::Negative(secs_before_epoch) => {
                            let date = Date::from_timestamp(secs_before_epoch).to_string();
                            diag_item = DiagItem::Item(date);
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

#[derive(Debug)]
enum DiagItem {
    Item(String),
    Group(String, String, Vec<DiagItem>, bool, Option<String>),
}

impl DiagItem {
    fn format(&self, annotate: bool) -> String {
        self.format_opt(0, "", annotate)
    }

    fn format_opt(&self, level: usize, separator: &str, annotate: bool) -> String {
        match self {
            DiagItem::Item(string) => {
                self.format_line(level, string, separator)
            },
            DiagItem::Group(_, _, _, _, _) => {
                if self.contains_group() || self.total_strings_len() > 20 || self.greatest_strings_len() > 20 {
                    self.multiline_composition(level, separator, annotate)
                } else {
                    self.single_line_composition(level, separator, annotate)
                }
            },
        }
    }

    fn format_line(&self, level: usize, string: &str, separator: &str) -> String {
        format!("{}{}{}", " ".repeat(level * 3), string, separator)
    }

    fn single_line_composition(&self, level: usize, separator: &str, annotate: bool) -> String {
        let string = match self {
            DiagItem::Item(s) => s.clone(),
            DiagItem::Group(begin, end, items, is_pairs, comment) => {
                let components: Vec<String> = items.iter().map(|item| {
                    match item {
                        DiagItem::Item(string) => string,
                        DiagItem::Group(_, _, _, _, _) => "<group>",
                    }.to_string()
                }).collect();
                let pair_separator = if *is_pairs { ": " } else { ", " };
                let s = flanked(&Self::joined(&components, ", ", Some(pair_separator)), begin, end);
                match (annotate, comment) {
                    (true, Some(comment)) => format!("{}   ; {}", s, comment),
                    _ => s,
                }
            },
        };
        self.format_line(level, &string, separator)
    }

    fn multiline_composition(&self, level: usize, separator: &str, annotate: bool) -> String {
        match self {
            DiagItem::Item(string) => string.to_owned(),
            DiagItem::Group(begin, end, items, is_pairs, comment) => {
                let mut lines: Vec<String> = vec![];
                let b = match (annotate, comment) {
                    (true, Some(comment)) => format!("{}   ; {}", begin, comment),
                    _ => begin.to_owned()
                };
                lines.push(self.format_line(level, &b, ""));
                for (index, item) in items.iter().enumerate() {
                    let separator = if index == items.len() - 1 {
                        ""
                    } else if *is_pairs && index & 1 == 0 {
                        ":"
                    } else {
                        ","
                    };
                    lines.push(item.format_opt(level + 1, separator, annotate));
                }
                lines.push(self.format_line(level, end, separator));
                lines.join("\n")
            },
        }
    }

    fn total_strings_len(&self) -> usize {
        match self {
            DiagItem::Item(string) => string.len(),
            DiagItem::Group(_, _, items, _, _) => {
                items.iter().fold(0, |acc, item| { acc + item.total_strings_len() })
            },
        }
    }

    fn greatest_strings_len(&self) -> usize {
        match self {
            DiagItem::Item(string) => string.len(),
            DiagItem::Group(_, _, items, _, _) => {
                items.iter().fold(0, |acc, item| { acc.max(item.total_strings_len()) })
            },
        }
    }

    fn is_group(&self) -> bool {
        matches!(self, DiagItem::Group(_, _, _, _, _))
    }

    fn contains_group(&self) -> bool {
        match self {
            DiagItem::Item(_) => false,
            DiagItem::Group(_, _, items, _, _) => {
                items.iter().any(|x| x.is_group())
            },
        }
    }

    fn joined(elements: &[String], item_separator: &str, pair_separator: Option<&str>) -> String {
        let pair_separator = pair_separator.unwrap_or(item_separator);
        let mut result = String::new();
        let len = elements.len();
        for (index, item) in elements.iter().enumerate() {
            result += item;
            if index != len - 1 {
                if index & 1 != 0 {
                    result += item_separator;
                } else {
                    result += pair_separator;
                }
            }
        }
        result
    }
}
