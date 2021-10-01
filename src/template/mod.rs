mod buffer;
pub(crate) use buffer::Buffer;

mod parse_error;
pub use parse_error::{ParseError, ParseErrorKind};

mod render;
use render::Render;

mod parse;
pub(crate) use parse::Parse;

mod shards;

mod fns;
pub use fns::Fns as TemplateFns;

mod structs;
use structs::Template;

mod global_context;
use global_context::GlobalContext;
pub use global_context::ALL_DATA;

#[derive(Debug, PartialEq)]
pub struct TemplateRoot {
    template: Template,
}

impl TemplateRoot {
    pub fn new<T>(s: T) -> Result<Self, ParseError>
    where
        T: Into<String>,
    {
        let s: String = s.into();
        let mut buffer = Buffer::new(s.into_bytes());
        let template = Template::parse(&mut buffer)?;
        Ok(Self { template })
    }

    pub fn render(&self, ctx: &GlobalContext, fns: &TemplateFns) -> String {
        let rendered = self.template.render(ctx, fns);

        // trim trailing spaces
        let mut rendered = rendered
            .lines()
            .map(|l| l.trim_end().to_owned())
            .collect::<Vec<_>>()
            .join("\n");

        // add missing NL at EOF if needed
        if !rendered.ends_with("\n") {
            rendered += "\n";
        }

        rendered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Message, MessageField, MessageFieldList, MessageFieldType, Node, NodeField, NodeFieldList,
        NodeFieldType,
    };

    const TEMPLATE: &str = "<helper file-header>

<each-node>
There is a node <helper node-name>
It has fields:
    <each-node-field>
        + <helper node-field-name> (printable: <if is-node-field-always-printable>YES<else>NO</if>)
    </each-node-field>
</each-node>

<each-message>
There is a message <helper message-name>
    <each-message-field>
        + <helper message-field-name> (is u8: <if is-message-field-u8>Y<else>N</if>)
    </each-message-field>
</each-message>
";

    const EXPECTED: &[&str] = &[
        "HEADER\n",
        "\n",
        "\n",
        "There is a node NodeOne\n",
        "It has fields:\n",
        "\n",
        "        + field1 (printable: YES)\n",
        "\n",
        "        + field2 (printable: YES)\n",
        "\n",
        "\n",
        "There is a node NodeTwo\n",
        "It has fields:\n",
        "\n",
        "        + field3 (printable: NO)\n",
        "\n",
        "\n",
        "\n",
        "\n",
        "There is a message Message1\n",
        "\n",
        "        + field1 (is u8: Y)\n",
        "\n",
        "        + field2 (is u8: N)\n",
        "\n",
    ];

    const NODES: &[Node] = &[
        Node {
            camelcase_name: "NodeOne",
            wqp_name: "node_one",
            fields: NodeFieldList(&[
                NodeField {
                    field_name: "field1",
                    field_type: NodeFieldType::Loc,
                    always_print: true,
                    comment: &["field 1 does this"],
                },
                NodeField {
                    field_name: "field2",
                    field_type: NodeFieldType::Loc,
                    always_print: true,
                    comment: &["field 2 does this"],
                },
            ]),
            comment: &["node one does this"],
        },
        Node {
            camelcase_name: "NodeTwo",
            wqp_name: "node_two",
            fields: NodeFieldList(&[NodeField {
                field_name: "field3",
                field_type: NodeFieldType::Loc,
                always_print: false,
                comment: &["field 3 does this"],
            }]),
            comment: &["node two does this"],
        },
    ];

    const MESSAGES: &[Message] = &[Message {
        camelcase_name: "Message1",
        fields: MessageFieldList(&[
            MessageField {
                name: "field1",
                field_type: MessageFieldType::Byte,
                comment: &[],
            },
            MessageField {
                name: "field2",
                field_type: MessageFieldType::Str,
                comment: &[],
            },
        ]),
        comment: &[],
    }];

    const CONTEXT: &GlobalContext = &GlobalContext {
        nodes: NODES,
        messages: MESSAGES,
    };

    fn file_header(_: &GlobalContext) -> String {
        format!("HEADER")
    }

    fn node_name(node: &Node) -> String {
        node.camelcase_name.to_string()
    }

    fn node_field_name(field: &NodeField) -> String {
        field.field_name.to_string()
    }

    fn is_node_field_always_printable(field: &NodeField) -> bool {
        field.always_print
    }

    fn message_name(message: &Message) -> String {
        message.camelcase_name.to_string()
    }

    fn message_field_name(message_field: &MessageField) -> String {
        message_field.name.to_string()
    }

    fn is_message_field_u8(message_field: &MessageField) -> bool {
        message_field.field_type == MessageFieldType::Byte
    }

    #[test]
    fn test_render() {
        let mut fns = TemplateFns::new();
        fns.register_helper("file-header", file_header);
        fns.register_helper("node-name", node_name);
        fns.register_helper("node-field-name", node_field_name);
        fns.register_predicate(
            "is-node-field-always-printable",
            is_node_field_always_printable,
        );
        fns.register_helper("message-name", message_name);
        fns.register_helper("message-field-name", message_field_name);
        fns.register_predicate("is-message-field-u8", is_message_field_u8);

        let template = TemplateRoot::new(TEMPLATE).unwrap();

        let rendered = template.render(CONTEXT, &fns);

        assert_eq!(EXPECTED.join(""), rendered);
    }
}