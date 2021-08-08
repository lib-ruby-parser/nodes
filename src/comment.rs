pub struct Comment<'a> {
    lines: &'static [&'static str],
    prefix: &'a str,
}

impl<'a> Comment<'a> {
    pub fn new(lines: &'static [&'static str], prefix: &'a str) -> Self {
        Self { lines, prefix }
    }

    pub fn to_string(&self, offset: usize) -> String {
        self.lines
            .iter()
            .map(|line| {
                let mut line = line.to_string();
                if !line.is_empty() {
                    line = format!(" {}", line);
                }
                format!(
                    "{spaces}{prefix}{line}",
                    spaces = " ".repeat(offset),
                    prefix = self.prefix,
                    line = line
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
