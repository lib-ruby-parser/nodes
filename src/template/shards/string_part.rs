use crate::template::{Buffer, Parse, ParseError, Render, TemplateFns};

pub(crate) trait StringPartBreakers: std::fmt::Debug + PartialEq {
    const BREAKERS: &'static [&'static str];
}

pub(crate) struct StringPart<T> {
    pub(crate) string: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T> StringPart<T> {
    pub(crate) fn new<S>(string: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            string: string.into(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for StringPart<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringPart")
            .field("string", &self.string)
            .finish()
    }
}

impl<T> PartialEq for StringPart<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl<Breakers> Parse for StringPart<Breakers>
where
    Breakers: StringPartBreakers,
{
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        let mut string = String::from("");

        while !buffer.is_eof() {
            let chunk = buffer.take(1).expect("bug: can't be EOF");
            string += &chunk;
            let mut abort = false;
            for breaker in Breakers::BREAKERS {
                if buffer.is(*breaker) {
                    abort = true;
                    break;
                }
            }
            if abort {
                break;
            }
        }

        Ok(Self::new(string))
    }
}

impl<T> Render<()> for StringPart<T> {
    fn render(&self, _ctx: &(), _fns: &TemplateFns) -> String {
        self.string.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Breakers;
    impl StringPartBreakers for Breakers {
        const BREAKERS: &'static [&'static str] = &["STOP HERE"];
    }

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new("a string STOP HERE".as_bytes().to_vec());
        let parsed = StringPart::<Breakers>::parse(&mut buffer).unwrap();

        assert_eq!(parsed, StringPart::new("a string "))
    }

    #[test]
    fn test_render() {
        let string = StringPart::<Breakers>::new("a string");
        let fns = TemplateFns::new();
        assert_eq!("a string", string.render(&(), &fns))
    }
}
