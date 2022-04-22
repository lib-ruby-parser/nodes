use crate::template::fns::Fns;
use crate::template::structs::{
    MessageFieldTemplatePart, MessageTemplatePart, NodeFieldTemplatePart, NodeTemplatePart,
    TemplatePart,
};
use crate::template::GlobalContext;
use crate::{Message, MessageField, Node, NodeField};

pub trait Render<Context>
where
    Context: ?Sized,
{
    fn render(&self, ctx: &Context, fns: &Fns) -> String;
}

impl Render<GlobalContext> for TemplatePart {
    fn render(&self, ctx: &GlobalContext, fns: &Fns) -> String {
        match self {
            Self::StringPart(string) => string.render(&(), fns),
            Self::GlobalHelper(helper) => helper.render(ctx, fns),
            Self::GlobalCondition(condition) => condition.render(ctx, fns),
            Self::NodesLoop(loop_) => loop_.render(ctx.nodes, fns),
            Self::MessagesLoop(loop_) => loop_.render(ctx.messages, fns),
        }
    }
}

impl Render<Node> for NodeTemplatePart {
    fn render(&self, node: &Node, fns: &Fns) -> String {
        match self {
            Self::StringPart(string) => string.render(&(), fns),
            Self::Helper(helper) => helper.render(node, fns),
            Self::Condition(condition) => condition.render(node, fns),
            Self::FieldsLoop(loop_) => loop_.render(node.fields, fns),
        }
    }
}

impl Render<NodeField> for NodeFieldTemplatePart {
    fn render(&self, node_field: &NodeField, fns: &Fns) -> String {
        match self {
            Self::StringPart(string) => string.render(&(), fns),
            Self::Helper(helper) => helper.render(node_field, fns),
            Self::Condition(condition) => condition.render(node_field, fns),
        }
    }
}

impl Render<Message> for MessageTemplatePart {
    fn render(&self, message: &Message, fns: &Fns) -> String {
        match self {
            Self::StringPart(string) => string.render(&(), fns),
            Self::Helper(helper) => helper.render(message, fns),
            Self::Condition(condition) => condition.render(message, fns),
            Self::FieldsLoop(loop_) => loop_.render(message.fields, fns),
        }
    }
}

impl Render<MessageField> for MessageFieldTemplatePart {
    fn render(&self, message_field: &MessageField, fns: &Fns) -> String {
        match self {
            Self::StringPart(string) => string.render(&(), fns),
            Self::Helper(helper) => helper.render(message_field, fns),
            Self::Condition(condition) => condition.render(message_field, fns),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::shards::{Condition, Helper, List, Loop, StringPart};
    use crate::template::Template;
    use crate::template::F;
    use crate::{MessageField, MessageFieldType, NodeField, NodeFieldType};

    static NODE1: Node = Node {
        camelcase_name: "One",
        wqp_name: "",
        fields: &[
            &NodeField {
                snakecase_name: "field1",
                field_type: NodeFieldType::Loc,
                always_print: true,
                comment: &[],
            },
            &NodeField {
                snakecase_name: "field2",
                field_type: NodeFieldType::Node,
                always_print: true,
                comment: &[],
            },
        ],
        comment: &[],
    };

    static NODE2: Node = Node {
        camelcase_name: "Two",
        wqp_name: "two",
        fields: &[
            &NodeField {
                snakecase_name: "field3",
                field_type: NodeFieldType::Loc,
                always_print: true,
                comment: &[],
            },
            &NodeField {
                snakecase_name: "field4",
                field_type: NodeFieldType::Node,
                always_print: false,
                comment: &[],
            },
        ],
        comment: &[],
    };

    static NODES: &[&Node] = &[&NODE1, &NODE2];

    static MESSAGE1: Message = Message {
        camelcase_name: "MessageOne",
        fields: &[&MessageField {
            snakecase_name: "field3",
            field_type: MessageFieldType::Byte,
            comment: &[],
        }],
        comment: &[],
    };

    static MESSAGES: &[&Message] = &[&MESSAGE1];

    static CONTEXT: &GlobalContext = &GlobalContext {
        nodes: NODES,
        messages: MESSAGES,
    };

    fn node_name(node: &Node) -> String {
        format!("{}", node.camelcase_name)
    }

    fn node_field_name(node_field: &NodeField) -> String {
        format!("{}", node_field.snakecase_name)
    }

    fn message_name(message: &Message) -> String {
        message.camelcase_name.to_owned()
    }

    fn message_field_name(message_field: &MessageField) -> String {
        message_field.snakecase_name.to_owned()
    }

    fn is_node_field_is_printable(node: &NodeField) -> bool {
        node.always_print
    }

    #[test]
    fn test_render() {
        let mut fns = Fns::new();
        fns.register::<Node, F::Helper>("node_name", node_name);
        fns.register::<NodeField, F::Helper>("node_field_name", node_field_name);
        fns.register::<NodeField, F::Predicate>(
            "is_node_field_is_printable",
            is_node_field_is_printable,
        );
        fns.register::<Message, F::Helper>("message_name", message_name);
        fns.register::<MessageField, F::Helper>("message_field_name", message_field_name);

        let template = Template::new([
            TemplatePart::StringPart(StringPart::new("\nheader\n")),
            TemplatePart::NodesLoop(Loop::new(Some(List::new([
                NodeTemplatePart::StringPart(StringPart::new("this is node ")),
                NodeTemplatePart::Helper(Helper::new("node_name")),
                NodeTemplatePart::StringPart(StringPart::new("\n    it has fields:\n")),
                NodeTemplatePart::FieldsLoop(Loop::new(Some(List::new([
                    NodeFieldTemplatePart::StringPart(StringPart::new("        + ")),
                    NodeFieldTemplatePart::Helper(Helper::new("node_field_name")),
                    NodeFieldTemplatePart::StringPart(StringPart::new(" (printable = ")),
                    NodeFieldTemplatePart::Condition(Condition::new(
                        String::from("is_node_field_is_printable"),
                        Some(List::new([NodeFieldTemplatePart::StringPart(
                            StringPart::new("YES"),
                        )])),
                        Some(List::new([NodeFieldTemplatePart::StringPart(
                            StringPart::new("NO"),
                        )])),
                    )),
                    NodeFieldTemplatePart::StringPart(StringPart::new(")\n")),
                ])))),
            ])))),
            TemplatePart::MessagesLoop(Loop::new(Some(List::new([
                MessageTemplatePart::StringPart(StringPart::new("message ")),
                MessageTemplatePart::Helper(Helper::new("message_name")),
                MessageTemplatePart::StringPart(StringPart::new("\n    with fields:\n")),
                MessageTemplatePart::FieldsLoop(Loop::new(Some(List::new([
                    MessageFieldTemplatePart::StringPart(StringPart::new("        - ")),
                    MessageFieldTemplatePart::Helper(Helper::new("message_field_name")),
                ])))),
            ])))),
        ]);

        let rendered = template.render(CONTEXT, &fns);
        assert_eq!(
            rendered,
            "
header
this is node One
    it has fields:
        + field1 (printable = YES)
        + field2 (printable = YES)
this is node Two
    it has fields:
        + field3 (printable = YES)
        + field4 (printable = NO)
message MessageOne
    with fields:
        - field3"
        )
    }
}
