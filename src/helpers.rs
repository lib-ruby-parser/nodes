pub fn camelcase_to_snakecase(s: &str) -> String {
    let mut words = vec![];
    let mut word = String::from("");

    for c in s.chars() {
        if c.is_uppercase() {
            // flush
            words.push(word);
            word = String::from("");
        }
        word.push(c);
    }

    words.push(word);

    words
        .into_iter()
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join("_")
        .to_lowercase()
}

#[test]
fn test_camelcase_to_snakecase() {
    assert_eq!(camelcase_to_snakecase("FooBar"), "foo_bar");
    assert_eq!(camelcase_to_snakecase("Foo"), "foo");
    assert_eq!(camelcase_to_snakecase("F"), "f");
}

pub fn escape_rust_keyword(s: &str) -> String {
    match s {
        "const" | "as" | "else" | "self" | "break" | "false" | "for" | "if" | "return" | "str"
        | "super" | "true" | "while" | "yield" => format!("{}_", s),
        _ => s.to_string(),
    }
}

#[test]
fn test_escape_rust_keyword() {
    assert_eq!(escape_rust_keyword("foo"), "foo");
    assert_eq!(escape_rust_keyword("while"), "while_");
}

pub fn snakecase_to_camelcase(s: &str) -> String {
    fn capitalize_word(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    s.split('_').map(capitalize_word).collect()
}

#[test]
fn test_snakecase_to_camelcase() {
    assert_eq!(snakecase_to_camelcase("foo_bar"), "FooBar");
    assert_eq!(snakecase_to_camelcase("foo"), "Foo");
    assert_eq!(snakecase_to_camelcase("f"), "F");
}
