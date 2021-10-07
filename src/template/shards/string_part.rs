use crate::template::{Buffer, Parse, Render, TemplateFns};

#[derive(Debug, PartialEq)]
pub struct StringPart {
    pub(crate) string: String,
}

impl StringPart {
    pub(crate) fn new<S>(string: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            string: string.into(),
        }
    }
}

impl Parse for StringPart {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        let mut string = String::from("");

        while !buffer.is_eof() {
            // start of directive
            if buffer.is("{{") {
                break;
            }

            string += &buffer.take(1).expect("bug: expected no EOF");
        }

        if string.is_empty() {
            None
        } else {
            Some(Self::new(string.replace("<dnl>\n", "")))
        }
    }
}

impl Render<()> for StringPart {
    fn render(&self, _ctx: &(), _fns: &TemplateFns) -> String {
        self.string.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new("a string {{".as_bytes().to_vec());
        let parsed = StringPart::parse(&mut buffer).unwrap();

        assert_eq!(parsed, StringPart::new("a string "))
    }

    #[test]
    fn test_render() {
        let string = StringPart::new("a string");
        let fns = TemplateFns::new();
        assert_eq!("a string", string.render(&(), &fns))
    }
}
