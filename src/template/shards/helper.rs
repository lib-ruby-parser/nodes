use crate::template::fns::FnSubject;
use crate::template::{render::Render, Buffer, Parse, ParseError, ParseErrorKind, TemplateFns};

#[derive(Debug, PartialEq)]
pub(crate) struct Helper {
    helper_name: String,
}

impl Helper {
    pub(crate) const START: &'static str = "<helper ";

    pub(crate) fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            helper_name: s.into(),
        }
    }

    fn validate_name(name: &str, buffer: &mut Buffer) -> Result<(), ParseError> {
        if name.is_empty()
            || name
                .chars()
                .any(|c| !c.is_alphanumeric() && c != '_' && c != '-')
        {
            // helper name must be [a-zA-Z9-0_]+
            return Err(ParseError {
                kind: ParseErrorKind::MissingHelperName,
                pos: buffer.pos(),
            });
        }

        Ok(())
    }
}

impl Parse for Helper {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        // skip "<helper "
        buffer
            .take(Self::START.len())
            .expect("bug: Helper::START is not in the buffer");

        // capture helper name
        let helper_name = buffer.take_until_pattern(">").ok_or_else(|| ParseError {
            kind: ParseErrorKind::MissingHelperName,
            pos: buffer.pos(),
        })?;
        Self::validate_name(&helper_name, buffer)?;

        // skip ">"
        if buffer.is(">") {
            buffer.take(1).expect("bug: '>' is not in the buffer");
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::MissingHelperClose,
                pos: buffer.pos(),
            });
        }
        Ok(Self::new(helper_name))
    }
}

impl<Context> Render<Context> for Helper
where
    Context: FnSubject,
{
    fn render(&self, ctx: &Context, fns: &TemplateFns) -> String {
        ctx.dispatch_helper(fns, &self.helper_name)
            .unwrap_or_else(|| panic!("Can't find helper {}", self.helper_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::global_context::{GlobalContext, NO_DATA};

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new("<helper foo>".as_bytes().to_vec());
        let parsed = Helper::parse(&mut buffer).unwrap();

        assert_eq!(parsed, Helper::new("foo"))
    }

    #[test]
    fn test_render() {
        let helper = Helper::new("foo");
        let mut fns = TemplateFns::new();
        fn foo(_: &GlobalContext) -> String {
            "bar".to_string()
        }
        fns.register_helper("foo", foo);
        assert_eq!("bar", helper.render(NO_DATA, &fns))
    }
}
