use crate::template::fns::{Bucket, GetRegistrySlice};
use crate::template::GlobalContext;
use crate::template::{render::Render, Buffer, Parse, ParseError, ParseErrorKind, TemplateFns};
use crate::{Message, MessageWithField, Node, NodeWithField};

#[derive(Debug, PartialEq)]
pub(crate) struct Condition<Branch> {
    predicate_name: String,
    if_true: Branch,
    if_false: Branch,
}

impl<Branch> Condition<Branch> {
    pub(crate) const START: &'static str = "<if ";
    const END: &'static str = "</if>";
    const ELSE: &'static str = "<else>";

    pub(crate) fn new<S>(predicate_name: S, if_true: Branch, if_false: Branch) -> Self
    where
        S: Into<String>,
    {
        Self {
            predicate_name: predicate_name.into(),
            if_true,
            if_false,
        }
    }
}

impl<Branch> Parse for Condition<Branch>
where
    Branch: Parse,
{
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        // skip "<if "
        buffer
            .take(Self::START.len())
            .expect("bug: Condition::START is not in the buffer");

        // capture predicate
        let predicate_name = buffer.take_until_pattern(">").ok_or_else(|| ParseError {
            kind: ParseErrorKind::MissingPredicateInCondition,
            pos: buffer.pos(),
        })?;

        // skip ">"
        if buffer.is(">") {
            buffer.take(1).expect("bug: '>' is not in the buffer");
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::MissingIfClosingTag,
                pos: buffer.pos(),
            });
        }

        // capture if-true body
        let if_true = {
            let bytes = buffer
                .take_until_pattern(Self::ELSE)
                .ok_or_else(|| ParseError {
                    kind: ParseErrorKind::MissingIfTrueBody,
                    pos: buffer.pos(),
                })?
                .into_bytes();
            let mut buffer = Buffer::new(bytes);
            Branch::parse(&mut buffer)?
        };

        // skip "<else>"
        if buffer.is(Self::ELSE) {
            buffer
                .take(Self::ELSE.len())
                .expect("bug: Condition::ELSE is not in the buffer");
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::MissingElse,
                pos: buffer.pos(),
            });
        }

        // capture if-false body
        let if_false = {
            let bytes = buffer
                .take_until_pattern(Self::END)
                .ok_or_else(|| ParseError {
                    kind: ParseErrorKind::MissingIfFalseBody,
                    pos: buffer.pos(),
                })?
                .into_bytes();
            let mut buffer = Buffer::new(bytes);
            Branch::parse(&mut buffer)?
        };

        // skip "</if>"
        if buffer.is(Self::END) {
            buffer
                .take(Self::END.len())
                .expect("bug: Condition::END is not in the buffer");
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::MissingIfClosingTag,
                pos: buffer.pos(),
            });
        }

        Ok(Self::new(predicate_name, if_true, if_false))
    }
}

impl<Branch> Render<Node> for Condition<Branch>
where
    Branch: Render<Node>,
{
    fn render(&self, node: &Node, fns: &TemplateFns) -> String {
        let bucket: &Bucket<Node> = fns.get_slice();
        if let Some(predicate) = bucket.get_predicate(&self.predicate_name) {
            if predicate(node) {
                self.if_true.render(node, fns)
            } else {
                self.if_false.render(node, fns)
            }
        } else {
            panic!("Can't find node predicate {}", self.predicate_name)
        }
    }
}

impl<Branch> Render<NodeWithField> for Condition<Branch>
where
    Branch: Render<NodeWithField>,
{
    fn render(&self, node_with_field: &NodeWithField, fns: &TemplateFns) -> String {
        let node_with_field_bucket: &Bucket<NodeWithField> = fns.get_slice();
        let node_bucket: &Bucket<Node> = fns.get_slice();

        if let Some(predicate) = node_with_field_bucket.get_predicate(&self.predicate_name) {
            if predicate(node_with_field) {
                self.if_true.render(node_with_field, fns)
            } else {
                self.if_false.render(node_with_field, fns)
            }
        } else if let Some(predicate) = node_bucket.get_predicate(&self.predicate_name) {
            if predicate(&node_with_field.node) {
                self.if_true.render(node_with_field, fns)
            } else {
                self.if_false.render(node_with_field, fns)
            }
        } else {
            panic!(
                "Can't find node_field/node predicate {}",
                self.predicate_name
            )
        }
    }
}

impl<Branch> Render<Message> for Condition<Branch>
where
    Branch: Render<Message>,
{
    fn render(&self, message: &Message, fns: &TemplateFns) -> String {
        let bucket: &Bucket<Message> = fns.get_slice();
        if let Some(predicate) = bucket.get_predicate(&self.predicate_name) {
            if predicate(message) {
                self.if_true.render(message, fns)
            } else {
                self.if_false.render(message, fns)
            }
        } else {
            panic!("Can't find message predicate {}", self.predicate_name)
        }
    }
}

impl<Branch> Render<MessageWithField> for Condition<Branch>
where
    Branch: Render<MessageWithField>,
{
    fn render(&self, message_with_field: &MessageWithField, fns: &TemplateFns) -> String {
        let message_with_field_bucket: &Bucket<MessageWithField> = fns.get_slice();
        let message_bucket: &Bucket<Message> = fns.get_slice();

        if let Some(predicate) = message_with_field_bucket.get_predicate(&self.predicate_name) {
            if predicate(message_with_field) {
                self.if_true.render(message_with_field, fns)
            } else {
                self.if_false.render(message_with_field, fns)
            }
        } else if let Some(predicate) = message_bucket.get_predicate(&self.predicate_name) {
            if predicate(&message_with_field.message) {
                self.if_true.render(message_with_field, fns)
            } else {
                self.if_false.render(message_with_field, fns)
            }
        } else {
            panic!(
                "Can't find message_field/message predicate {}",
                self.predicate_name
            )
        }
    }
}

impl<Branch> Render<GlobalContext> for Condition<Branch>
where
    Branch: Render<GlobalContext>,
{
    fn render(&self, ctx: &GlobalContext, fns: &TemplateFns) -> String {
        let bucket: &Bucket<GlobalContext> = fns.get_slice();
        if let Some(predicate) = bucket.get_predicate(&self.predicate_name) {
            if predicate(ctx) {
                self.if_true.render(ctx, fns)
            } else {
                self.if_false.render(ctx, fns)
            }
        } else {
            panic!("Can't find global predicate {}", self.predicate_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::template::{
        global_context::NO_DATA,
        shards::StringPart,
        structs::{Template, TemplatePart},
        GlobalContext,
    };

    use super::*;

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new("<if foo>1<else>2</if>".as_bytes().to_vec());
        let parsed = Condition::<Template>::parse(&mut buffer).unwrap();

        assert_eq!(
            parsed,
            Condition::new(
                "foo",
                Template::new([TemplatePart::StringPart(StringPart::new("1")),]),
                Template::new([TemplatePart::StringPart(StringPart::new("2")),])
            )
        )
    }

    #[test]
    fn test_render() {
        let condition = Condition::new(
            "foo",
            Template::new([TemplatePart::StringPart(StringPart::new("1"))]),
            Template::new([TemplatePart::StringPart(StringPart::new("2"))]),
        );

        let mut fns = TemplateFns::new();
        fn always_true(_: &GlobalContext) -> bool {
            true
        }
        fns.register_predicate("foo", always_true);
        assert_eq!("1", condition.render(NO_DATA, &fns));

        let mut fns = TemplateFns::new();
        fn always_false(_: &GlobalContext) -> bool {
            false
        }
        fns.register_predicate("foo", always_false);
        assert_eq!("2", condition.render(NO_DATA, &fns));
    }
}
