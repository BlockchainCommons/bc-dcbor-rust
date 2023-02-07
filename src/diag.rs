use crate::{CBOR, known_tags::KnownTags};

impl CBOR {
    pub fn diagnostic_annotated(&self, annotate: bool, known_tags: Option<Box<dyn KnownTags>>) -> String {
        self.diag_item(annotate, known_tags).format(annotate)
    }

    pub fn diagnostic(&self) -> String {
        self.diagnostic_annotated(false, None)
    }

    fn diag_item(&self, annotate: bool, known_tags: Option<Box<dyn KnownTags>>) -> DiagItem {
        todo!()
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
