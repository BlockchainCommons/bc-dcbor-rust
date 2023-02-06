use std::hash::Hash;


#[derive(Debug, Clone)]
pub struct Tag {
    value: u64,
    name: Option<String>,
}

impl Tag {
    pub fn new(value: u64, name: Option<&str>) -> Tag {
        let name = match name {
            None => None,
            Some(name) => Some(name.to_string())
        };
        Tag { value, name }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Tag { }

impl Hash for Tag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name().clone().unwrap_or_else(|| self.value().to_string()))
    }
}
