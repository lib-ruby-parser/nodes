use crate::template::fns::FnSubject;
use crate::template::{render::Render, shards::FnName, Buffer, Parse, TemplateFns};

#[derive(Debug, PartialEq)]
pub struct Helper {
    helper_name: String,
}

impl Helper {
    pub(crate) fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            helper_name: s.into(),
        }
    }
}

impl Parse for Helper {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        // consume "{{ helper "
        if !buffer.is("{{ helper ") {
            return None;
        }
        buffer.consume("{{ helper ");

        // capture helper name
        let start = buffer.pos();
        let helper_name = FnName::parse(buffer)
            .unwrap_or_else(|| panic!("helper name is empty at pos {}", start))
            .unwrap();

        // consume " }}"
        if !buffer.is(" }}") {
            panic!("{{ helper }} is not closed at {}", buffer.pos());
        }
        buffer.consume(" }}");
        Some(Self::new(helper_name))
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
        let mut buffer = Buffer::new("{{ helper foo }}".as_bytes().to_vec());
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
