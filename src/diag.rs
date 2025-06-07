import_stdlib!();

use super::string_util::flanked;
use crate::{
    CBOR, CBORCase, TagsStoreOpt, tags_store::TagsStoreTrait,
};

#[derive(Clone, Default)]
pub struct DiagFormatOpts<'a> {
    annotate: bool,
    summarize: bool,
    flat: bool,
    tags: TagsStoreOpt<'a>,
}

impl<'a> DiagFormatOpts<'a> {
    /// Sets whether to annotate the diagnostic notation with tags.
    pub fn annotate(mut self, annotate: bool) -> Self {
        self.annotate = annotate;
        self
    }

    /// Sets whether to summarize the diagnostic notation.
    pub fn summarize(mut self, summarize: bool) -> Self {
        self.summarize = summarize;
        self
    }

    /// Sets whether to format the diagnostic notation in a flat manner.
    pub fn flat(mut self, flat: bool) -> Self {
        self.flat = flat;
        self
    }

    /// Sets the formatting context for the diagnostic notation.
    pub fn context(mut self, tags: TagsStoreOpt<'a>) -> Self {
        self.tags = tags;
        self
    }
}

/// Affordances for viewing CBOR in diagnostic notation.
impl CBOR {
    /// Returns a representation of this CBOR in diagnostic notation.
    ///
    /// Optionally annotates the output, e.g. formatting dates and adding names
    /// of known tags.
    pub fn diagnostic_opt(&self, opts: DiagFormatOpts<'_>) -> String {
        self.diag_item(opts.clone()).format(opts)
    }

    /// Returns a representation of this CBOR in diagnostic notation.
    pub fn diagnostic(&self) -> String {
        self.diagnostic_opt(DiagFormatOpts::default())
    }

    /// Returns a representation of this CBOR in diagnostic notation, with
    /// annotations.
    pub fn diagnostic_annotated(&self) -> String {
        self.diagnostic_opt(DiagFormatOpts::default().annotate(true))
    }

    pub fn diagnostic_flat(&self) -> String {
        self.diagnostic_opt(DiagFormatOpts::default().flat(true))
    }

    pub fn summary(&self) -> String {
        self.diagnostic_opt(DiagFormatOpts::default().summarize(true))
    }

    pub fn summary_opt(&self, tags: &dyn TagsStoreTrait) -> String {
        self.diagnostic_opt(
            DiagFormatOpts::default()
                .summarize(true)
                .context(TagsStoreOpt::Custom(tags)),
        )
    }

    fn diag_item(&self, opts: DiagFormatOpts<'_>) -> DiagItem {
        match self.as_case() {
            CBORCase::Unsigned(_)
            | CBORCase::Negative(_)
            | CBORCase::ByteString(_)
            | CBORCase::Text(_)
            | CBORCase::Simple(_) => DiagItem::Item(format!("{}", self)),

            CBORCase::Array(a) => {
                let begin = "[".to_string();
                let end = "]".to_string();
                let items =
                    a.iter().map(|x| x.diag_item(opts.clone())).collect();
                let is_pairs = false;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            }
            CBORCase::Map(m) => {
                let begin = "{".to_string();
                let end = "}".to_string();
                let items = m
                    .iter()
                    .flat_map(|(key, value)| {
                        vec![
                            key.diag_item(opts.clone()),
                            value.diag_item(opts.clone()),
                        ]
                    })
                    .collect();
                let is_pairs = true;
                let comment = None;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            }
            CBORCase::Tagged(tag, item) => {
                if opts.summarize {
                    if let TagsStoreOpt::Custom(tags_store_trait) = &opts.tags {
                        if let Some(summarizer) = tags_store_trait.summarizer(tag.value()) {
                            match summarizer(item.clone()) {
                                Ok(summary) => {
                                    return DiagItem::Item(summary);
                                }
                                Err(error) => {
                                    return DiagItem::Item(format!(
                                        "<error: {}>",
                                        error
                                    ));
                                }
                            }
                        }
                    }
                }

                // Get a possible comment before we move opts
                let comment = if opts.annotate {
                    if let TagsStoreOpt::Custom(tags_store_trait) = &opts.tags {
                        tags_store_trait.assigned_name_for_tag(tag)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let diag_item = item.diag_item(opts);
                let begin = tag.value().to_string() + "(";
                let end = ")".to_string();
                let items = vec![diag_item];
                let is_pairs = false;
                DiagItem::Group(begin, end, items, is_pairs, comment)
            }
        }
    }
}

#[derive(Debug)]
enum DiagItem {
    Item(String),
    Group(String, String, Vec<DiagItem>, bool, Option<String>),
}

impl DiagItem {
    fn format(&self, opts: DiagFormatOpts<'_>) -> String {
        self.format_opt(0, "", opts)
    }

    fn format_opt(
        &self,
        level: usize,
        separator: &str,
        opts: DiagFormatOpts<'_>,
    ) -> String {
        match self {
            DiagItem::Item(string) => {
                self.format_line(level, opts, string, separator, None)
            }
            DiagItem::Group(_, _, _, _, _) => {
                if !opts.flat
                    && (self.contains_group()
                        || self.total_strings_len() > 20
                        || self.greatest_strings_len() > 20)
                {
                    self.multiline_composition(level, separator, opts)
                } else {
                    self.single_line_composition(level, separator, opts)
                }
            }
        }
    }

    fn format_line(
        &self,
        level: usize,
        opts: DiagFormatOpts<'_>,
        string: &str,
        separator: &str,
        comment: Option<&str>,
    ) -> String {
        let indent = if opts.flat {
            "".to_string()
        } else {
            " ".repeat(level * 4)
        };
        let result = format!("{}{}{}", indent, string, separator);
        if let Some(comment) = comment {
            format!("{}   / {} /", result, comment)
        } else {
            result
        }
    }

    fn single_line_composition(
        &self,
        level: usize,
        separator: &str,
        opts: DiagFormatOpts<'_>,
    ) -> String {
        let string: String;
        let comment: Option<&str>;
        match self {
            DiagItem::Item(s) => {
                string = s.clone();
                comment = None;
            }
            DiagItem::Group(begin, end, items, is_pairs, comm) => {
                let components: Vec<String> = items
                    .iter()
                    .map(|item| match item {
                        DiagItem::Item(string) => string.clone(),
                        DiagItem::Group(_, _, _, _, _) => item
                            .single_line_composition(
                                level + 1,
                                separator,
                                opts.clone(),
                            ),
                    })
                    .collect();
                let pair_separator = if *is_pairs { ": " } else { ", " };
                string = flanked(
                    &Self::joined(&components, ", ", Some(pair_separator)),
                    begin,
                    end,
                );
                comment = comm.as_ref().map(|x| x.as_str());
            }
        };
        self.format_line(level, opts, &string, separator, comment)
    }

    fn multiline_composition(
        &self,
        level: usize,
        separator: &str,
        opts: DiagFormatOpts<'_>,
    ) -> String {
        match self {
            DiagItem::Item(string) => string.to_owned(),
            DiagItem::Group(begin, end, items, is_pairs, comment) => {
                let mut lines: Vec<String> = vec![];
                lines.push(self.format_line(
                    level,
                    opts.clone().flat(false),
                    begin,
                    "",
                    comment.as_ref().map(|x| x.as_str()),
                ));
                for (index, item) in items.iter().enumerate() {
                    let separator = if index == items.len() - 1 {
                        ""
                    } else if *is_pairs && index & 1 == 0 {
                        ":"
                    } else {
                        ","
                    };
                    lines.push(item.format_opt(
                        level + 1,
                        separator,
                        opts.clone(),
                    ));
                }
                lines.push(self.format_line(level, opts, end, separator, None));
                lines.join("\n")
            }
        }
    }

    fn total_strings_len(&self) -> usize {
        match self {
            DiagItem::Item(string) => string.len(),
            DiagItem::Group(_, _, items, _, _) => items
                .iter()
                .fold(0, |acc, item| acc + item.total_strings_len()),
        }
    }

    fn greatest_strings_len(&self) -> usize {
        match self {
            DiagItem::Item(string) => string.len(),
            DiagItem::Group(_, _, items, _, _) => items
                .iter()
                .fold(0, |acc, item| acc.max(item.total_strings_len())),
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
            }
        }
    }

    fn joined(
        elements: &[String],
        item_separator: &str,
        pair_separator: Option<&str>,
    ) -> String {
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
