import_stdlib!();

use crate::{tags_store::TagsStoreTrait, with_tags, CBORCase, CBOR};

use super::string_util::flanked;

/// Affordances for viewing CBOR in diagnostic notation.
impl CBOR {
    /// Returns a representation of this CBOR in diagnostic notation.
    ///
    /// Optionally annotates the output, e.g. formatting dates and adding names
    /// of known tags.
    pub fn diagnostic_opt(&self, annotate: bool, summarize: bool, tags: Option<&dyn TagsStoreTrait>) -> String {
        self.diag_item(annotate, summarize, tags).format(annotate)
    }

    /// Returns a representation of this CBOR in diagnostic notation.
    pub fn diagnostic(&self) -> String {
        self.diagnostic_opt(false, false, None)
    }

    /// Returns a representation of this CBOR in diagnostic notation, with annotations.
    pub fn diagnostic_annotated(&self) -> String {
        with_tags!(|tags: &dyn TagsStoreTrait| {
            self.diagnostic_opt(true, false, Some(tags))
        })
    }

    pub fn summary(&self) -> String {
        with_tags!(|tags: &dyn TagsStoreTrait| {
            self.diagnostic_opt(false, true, Some(tags))
        })
    }

    pub fn summary_opt(&self, tags: &dyn TagsStoreTrait) -> String {
        self.diagnostic_opt(false, true, Some(tags))
    }

    fn diag_item(&self, annotate: bool, summarize: bool, tags: Option<&dyn TagsStoreTrait>) -> DiagItem {
        match self.as_case() {
            CBORCase::Unsigned(_) | CBORCase::Negative(_) | CBORCase::ByteString(_) |
            CBORCase::Text(_) | CBORCase::Simple(_) => DiagItem::Item(format!("{}", self)),

            CBORCase::Array(a) => {
                let begin = "[".to_string();
                let end = "]".to_string();
                let items = a.iter().map(|x| x.diag_item(annotate, summarize, tags)).collect();
                let is_pairs = false;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            },
            CBORCase::Map(m) => {
                let begin = "{".to_string();
                let end = "}".to_string();
                let items = m.iter().flat_map(|(key, value)| vec![
                    key.diag_item(annotate, summarize, tags),
                    value.diag_item(annotate, summarize, tags)
                ]).collect();
                let is_pairs = true;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            },
            CBORCase::Tagged(tag, item) => {
                if summarize {
                    if let Some(tags) = tags {
                        if let Some(summarizer) = tags.summarizer(tag.value()) {
                            match summarizer(item.clone()) {
                                Ok(summary) => return DiagItem::Item(summary),
                                Err(error) => return DiagItem::Item(format!("<error: {}>", error)),
                            }
                        }
                    }
                }
                let diag_item = item.diag_item(annotate, summarize, tags);
                let begin = tag.value().to_string() + "(";
                let end = ")".to_string();
                let items = vec![diag_item];
                let is_pairs = false;
                let comment = if annotate {
                    tags.as_ref().and_then(|x| x.assigned_name_for_tag(tag))
                } else {
                    None
                };
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
                self.format_line(level, string, separator, None)
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

    fn format_line(&self, level: usize, string: &str, separator: &str, comment: Option<&str>) -> String {
        let result = format!("{}{}{}", " ".repeat(level * 4), string, separator);
        if let Some(comment) = comment {
            format!("{}   / {} /", result, comment)
        } else {
            result
        }
    }

    fn single_line_composition(&self, level: usize, separator: &str, _annotate: bool) -> String {
        let string: String;
        let comment: Option<&str>;
        match self {
            DiagItem::Item(s) => {
                string = s.clone();
                comment = None;
            },
            DiagItem::Group(begin, end, items, is_pairs, comm) => {
                let components: Vec<String> = items.iter().map(|item| {
                    match item {
                        DiagItem::Item(string) => string,
                        DiagItem::Group(_, _, _, _, _) => "<group>",
                    }.to_string()
                }).collect();
                let pair_separator = if *is_pairs { ": " } else { ", " };
                string = flanked(&Self::joined(&components, ", ", Some(pair_separator)), begin, end);
                comment = comm.as_ref().map(|x| x.as_str());
            },
        };
        self.format_line(level, &string, separator, comment)
    }

    fn multiline_composition(&self, level: usize, separator: &str, annotate: bool) -> String {
        match self {
            DiagItem::Item(string) => string.to_owned(),
            DiagItem::Group(begin, end, items, is_pairs, comment) => {
                let mut lines: Vec<String> = vec![];
                lines.push(self.format_line(level, begin, "", comment.as_ref().map(|x| x.as_str())));
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
                lines.push(self.format_line(level, end, separator, None));
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
