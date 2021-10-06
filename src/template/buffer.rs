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

    pub(crate) fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.bytes[self.pos..].is_empty()
    }

    pub(crate) fn is(&self, pattern: &str) -> bool {
        self.bytes[self.pos..].starts_with(pattern.as_bytes())
    }

    pub(crate) fn consume(&mut self, pattern: &str) {
        if self.is(pattern) {
            self.take(pattern.len());
        } else {
            panic!("expected to get {:?} at {}", pattern, self.pos)
        }
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

    pub(crate) fn take_char(&mut self) -> Option<char> {
        self.take(1).map(|s| s.chars().next().unwrap())
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
}
