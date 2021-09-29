use crate::template::fns::{Bucket, FnSubject, GetRegistrySlice};
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
        let bucket: &Bucket<Context> = fns.get_slice();
        let helper = bucket.get_helper(&self.helper_name);
        helper(ctx)
    }
}
