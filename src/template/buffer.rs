pub(crate) struct Buffer {
    bytes: Vec<u8>,
    pos: usize,
}

impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Buffer {:?} (pos = {}, size = {})",
            &self.bytes[self.pos..std::cmp::min(self.pos + 5, self.bytes.len())],
            self.pos,
            self.bytes.len()
        )
    }
}

impl Buffer {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, pos: 0 }
    }

    pub(crate) fn pos(&self) -> usize {
        self.pos
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.bytes[self.pos..].is_empty()
    }

    pub(crate) fn is(&self, pattern: &str) -> bool {
        self.bytes[self.pos..].starts_with(pattern.as_bytes())
    }

    pub(crate) fn take(&mut self, n: usize) -> Option<String> {
        let rest = &self.bytes[self.pos..];
        if rest.len() >= n {
            let result = rest[..n].to_vec();
            self.pos += n;
            Some(String::from_utf8(result).expect("template string is not utf-8"))
        } else {
            None
        }
    }

    pub(crate) fn take_until_pattern(&mut self, pattern: &str) -> Option<String> {
        for i in self.pos..self.bytes.len() {
            if self.bytes[i..].starts_with(pattern.as_bytes()) {
                return self.take(i - self.pos);
            }
        }
        // take all
        self.take(self.bytes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;

    const S: &str = "foo\nbar";

    fn new_buffer() -> Buffer {
        Buffer::new(S.as_bytes().to_vec())
    }

    #[test]
    fn test_is() {
        let buffer = new_buffer();
        assert!(buffer.is("foo"));
        assert!(buffer.is("foo\nbar"));
        assert!(!buffer.is("something else"));
    }

    #[test]
    fn test_take() {
        let mut buffer = new_buffer();
        assert!(!buffer.is_eof());
        assert_eq!(buffer.take(1).unwrap(), "f");
        assert!(!buffer.is_eof());
        assert_eq!(buffer.take(2).unwrap(), "oo");
        assert!(!buffer.is_eof());
        assert_eq!(buffer.take(10), None);
        assert_eq!(buffer.take(4).unwrap(), "\nbar");
        assert!(buffer.is_eof());
    }

    #[test]
    fn test_take_until_pattern() {
        let mut buffer = new_buffer();
        assert_eq!(buffer.take_until_pattern("bar").unwrap(), "foo\n");

        let mut buffer = new_buffer();
        assert_eq!(buffer.take_until_pattern("unknown").unwrap(), "foo\nbar");
    }
}
