import_stdlib!();

pub fn flanked(s: &str, left: &str, right: &str) -> String {
    left.to_owned() + s + right
}

pub fn is_printable(c: char) -> bool {
    !c.is_ascii() || (32..=126).contains(&(c as u32))
}

pub fn sanitized(string: &str) -> Option<String> {
    let mut has_printable = false;
    let chars: Vec<_> = string
        .chars()
        .map(|c| {
            if is_printable(c) {
                has_printable = true;
                c
            } else {
                '.'
            }
        })
        .collect();
    if !has_printable {
        None
    } else {
        Some(chars.into_iter().collect())
    }
}
