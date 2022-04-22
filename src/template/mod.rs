mod buffer;
pub(crate) use buffer::Buffer;

mod render;
use render::Render;

mod parse;
pub(crate) use parse::Parse;

mod shards;

mod fns;
pub(crate) use fns::Dispatch;
pub use fns::Fns as TemplateFns;
pub use fns::F;

mod structs;
use structs::{NodeTemplate, Template};

mod global_context;
pub use global_context::GlobalContext;
pub use global_context::ALL_DATA;

pub trait PublicTemplate {
    type Context;
    type InnerTemplate: Render<Self::Context> + Parse;

    fn inner(&self) -> &Self::InnerTemplate;
    fn build(inner: Self::InnerTemplate) -> Self
    where
        Self: Sized;

    fn new<T>(s: T) -> Option<Self>
    where
        T: Into<String>,
        Self: Sized,
    {
        let s: String = s.into();
        let mut buffer = Buffer::new(s.into_bytes());
        let inner = Self::InnerTemplate::parse(&mut buffer)?;
        Some(Self::build(inner))
    }

    fn render(&self, ctx: &Self::Context, fns: &TemplateFns) -> String {
        let rendered = self.inner().render(ctx, fns);

        // trim trailing spaces
        let mut rendered = rendered
            .lines()
            .map(|l| l.trim_end().to_owned())
            .collect::<Vec<_>>()
            .join("\n");

        // add missing NL at EOF if needed
        if !rendered.ends_with('\n') {
            rendered += "\n";
        }

        rendered
    }
}

#[derive(Debug, PartialEq)]
pub struct TemplateRoot {
    inner: Template,
}

impl PublicTemplate for TemplateRoot {
    type Context = GlobalContext;
    type InnerTemplate = Template;

    fn inner(&self) -> &Self::InnerTemplate {
        &self.inner
    }

    fn build(inner: Self::InnerTemplate) -> Self {
        Self { inner }
    }
}

#[derive(Debug, PartialEq)]
pub struct NodeTemplateRoot {
    inner: NodeTemplate,
}

impl PublicTemplate for NodeTemplateRoot {
    type Context = crate::Node;
    type InnerTemplate = NodeTemplate;

    fn inner(&self) -> &Self::InnerTemplate {
        &self.inner
    }

    fn build(inner: Self::InnerTemplate) -> Self {
        Self { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        template::fns::F, Message, MessageField, MessageFieldType, Node, NodeField, NodeFieldType,
    };

    const TEMPLATE: &str = "{{ helper file-header }}

{{ each node }}<dnl>
There is a node {{ helper node-name }}
    It has fields:
{{ each node-field }}<dnl>
        + {{ helper node-field-name }} (printable: {{ if is-node-field-always-printable }}YES{{ else }}NO{{ end }}, node name is still {{ helper node-name }})
{{ end }}<dnl>
{{ end }}<dnl>

{{ each message }}<dnl>
There is a message {{ helper message-name }}
{{ each message-field }}<dnl>
        + {{ helper message-field-name }} (is u8: {{ if is-message-field-u8 }}Y{{ else }}N{{ end }}, message name is still {{ helper message-name }})
{{ end }}<dnl>
{{ end }}<dnl>
";

    const EXPECTED: &[&str] = &[
        "HEADER\n",
        "\n",
        "There is a node NodeOne\n",
        "    It has fields:\n",
        "        + field1 (printable: YES, node name is still NodeOne)\n",
        "        + field2 (printable: YES, node name is still NodeOne)\n",
        "There is a node NodeTwo\n",
        "    It has fields:\n",
        "        + field3 (printable: NO, node name is still NodeTwo)\n",
        "\n",
        "There is a message Message1\n",
        "        + field1 (is u8: Y, message name is still Message1)\n",
        "        + field2 (is u8: N, message name is still Message1)\n",
    ];

    static NODE_ONE: Node = Node {
        camelcase_name: "NodeOne",
        wqp_name: "node_one",
        fields: &[
            &NodeField {
                snakecase_name: "field1",
                field_type: NodeFieldType::Loc,
                always_print: true,
                comment: &["field 1 does this"],
            },
            &NodeField {
                snakecase_name: "field2",
                field_type: NodeFieldType::Loc,
                always_print: true,
                comment: &["field 2 does this"],
            },
        ],
        comment: &["node one does this"],
    };

    static NODE_TWO: Node = Node {
        camelcase_name: "NodeTwo",
        wqp_name: "node_two",
        fields: &[&NodeField {
            snakecase_name: "field3",
            field_type: NodeFieldType::Loc,
            always_print: false,
            comment: &["field 3 does this"],
        }],
        comment: &["node two does this"],
    };

    static NODES: &[&Node] = &[&NODE_ONE, &NODE_TWO];

    static MESSAGE_ONE: Message = Message {
        camelcase_name: "Message1",
        fields: &[
            &MessageField {
                snakecase_name: "field1",
                field_type: MessageFieldType::Byte,
                comment: &[],
            },
            &MessageField {
                snakecase_name: "field2",
                field_type: MessageFieldType::Str,
                comment: &[],
            },
        ],
        comment: &[],
    };
    static MESSAGES: &[&Message] = &[&MESSAGE_ONE];

    static CONTEXT: &GlobalContext = &GlobalContext {
        nodes: NODES,
        messages: MESSAGES,
    };

    fn file_header(_: &GlobalContext) -> String {
        format!("HEADER")
    }

    fn node_name(node: &Node) -> String {
        node.camelcase_name.to_string()
    }

    fn node_field_name(node_field: &NodeField) -> String {
        node_field.snakecase_name.to_string()
    }

    fn is_node_field_always_printable(node_field: &NodeField) -> bool {
        node_field.always_print
    }

    fn message_name(message: &Message) -> String {
        message.camelcase_name.to_string()
    }

    fn message_field_name(message_field: &MessageField) -> String {
        message_field.snakecase_name.to_string()
    }

    fn is_message_field_u8(message_field: &MessageField) -> bool {
        message_field.field_type == MessageFieldType::Byte
    }

    #[test]
    fn test_render() {
        let mut fns = TemplateFns::new();
        fns.register::<GlobalContext, F::Helper>("file-header", file_header);
        fns.register::<Node, F::Helper>("node-name", node_name);
        fns.register::<NodeField, F::Helper>("node-field-name", node_field_name);
        fns.register::<NodeField, F::Predicate>(
            "is-node-field-always-printable",
            is_node_field_always_printable,
        );
        fns.register::<Message, F::Helper>("message-name", message_name);
        fns.register::<MessageField, F::Helper>("message-field-name", message_field_name);
        fns.register::<MessageField, F::Predicate>("is-message-field-u8", is_message_field_u8);

        let template = TemplateRoot::new(TEMPLATE).unwrap();

        let rendered = template.render(CONTEXT, &fns);

        assert_eq!(EXPECTED.join(""), rendered);
    }
}
