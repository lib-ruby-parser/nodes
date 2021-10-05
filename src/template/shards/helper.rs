use crate::template::fns::{Bucket, GetRegistrySlice};
use crate::template::GlobalContext;
use crate::template::{render::Render, Buffer, Parse, ParseError, ParseErrorKind, TemplateFns};
use crate::{Message, MessageWithField, Node, NodeWithField};

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

impl Render<Node> for Helper {
    fn render(&self, node: &Node, fns: &TemplateFns) -> String {
        if let Some(f) = fns.get_slice().get_helper(&self.helper_name) {
            f(node)
        } else {
            panic!("Can't find node helper {}", self.helper_name)
        }
    }
}

impl Render<NodeWithField> for Helper {
    fn render(&self, node_with_field: &NodeWithField, fns: &TemplateFns) -> String {
        let node_with_field_bucket: &Bucket<NodeWithField> = fns.get_slice();
        let node_bucket: &Bucket<Node> = fns.get_slice();

        if let Some(f) = node_with_field_bucket.get_helper(&self.helper_name) {
            f(node_with_field)
        } else if let Some(f) = node_bucket.get_helper(&self.helper_name) {
            f(&node_with_field.node)
        } else {
            panic!("Can't find node/node_field helper {}", self.helper_name)
        }
    }
}

impl Render<Message> for Helper {
    fn render(&self, message: &Message, fns: &TemplateFns) -> String {
        if let Some(f) = fns.get_slice().get_helper(&self.helper_name) {
            f(message)
        } else {
            panic!("Can't find message helper {}", self.helper_name)
        }
    }
}

impl Render<MessageWithField> for Helper {
    fn render(&self, message_with_field: &MessageWithField, fns: &TemplateFns) -> String {
        let message_with_field_bucket: &Bucket<MessageWithField> = fns.get_slice();
        let message_bucket: &Bucket<Message> = fns.get_slice();

        if let Some(f) = message_with_field_bucket.get_helper(&self.helper_name) {
            f(message_with_field)
        } else if let Some(f) = message_bucket.get_helper(&self.helper_name) {
            f(&message_with_field.message)
        } else {
            panic!(
                "Can't find message/message_field helper {}",
                self.helper_name
            )
        }
    }
}

impl Render<GlobalContext> for Helper {
    fn render(&self, ctx: &GlobalContext, fns: &TemplateFns) -> String {
        if let Some(f) = fns.get_slice().get_helper(&self.helper_name) {
            f(ctx)
        } else {
            panic!("Can't find global helper {}", self.helper_name)
        }
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
