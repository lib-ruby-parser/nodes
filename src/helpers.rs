pub fn camel_case_to_underscored(s: &str) -> String {
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
}
