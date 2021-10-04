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
pub use global_context::GlobalContext;
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
        if !rendered.ends_with('\n') {
            rendered += "\n";
        }

        rendered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Message, MessageField, MessageFieldList, MessageFieldType, MessageWithField, Node,
        NodeField, NodeFieldList, NodeFieldType, NodeWithField,
    };

    const TEMPLATE: &str = "<helper file-header>

<each-node><dnl>
There is a node <helper node-name>
    It has fields:
<each-node-field><dnl>
        + <helper node-field-name> (printable: <if is-node-field-always-printable>YES<else>NO</if>)
</each-node-field><dnl>
</each-node><dnl>

<each-message><dnl>
There is a message <helper message-name>
<each-message-field><dnl>
        + <helper message-field-name> (is u8: <if is-message-field-u8>Y<else>N</if>)
</each-message-field><dnl>
</each-message><dnl>
";

    const EXPECTED: &[&str] = &[
        "HEADER\n",
        "\n",
        "There is a node NodeOne\n",
        "    It has fields:\n",
        "        + field1 (printable: YES)\n",
        "        + field2 (printable: YES)\n",
        "There is a node NodeTwo\n",
        "    It has fields:\n",
        "        + field3 (printable: NO)\n",
        "\n",
        "There is a message Message1\n",
        "        + field1 (is u8: Y)\n",
        "        + field2 (is u8: N)\n",
    ];

    const NODES: &[Node] = &[
        Node {
            camelcase_name: "NodeOne",
            wqp_name: "node_one",
            fields: NodeFieldList(&[
                NodeField {
                    snakecase_name: "field1",
                    field_type: NodeFieldType::Loc,
                    always_print: true,
                    comment: &["field 1 does this"],
                },
                NodeField {
                    snakecase_name: "field2",
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
                snakecase_name: "field3",
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
                snakecase_name: "field1",
                field_type: MessageFieldType::Byte,
                comment: &[],
            },
            MessageField {
                snakecase_name: "field2",
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

    fn node_field_name(node: &NodeWithField) -> String {
        node.field.snakecase_name.to_string()
    }

    fn is_node_field_always_printable(node: &NodeWithField) -> bool {
        node.field.always_print
    }

    fn message_name(message: &Message) -> String {
        message.camelcase_name.to_string()
    }

    fn message_field_name(message: &MessageWithField) -> String {
        message.field.snakecase_name.to_string()
    }

    fn is_message_field_u8(message: &MessageWithField) -> bool {
        message.field.field_type == MessageFieldType::Byte
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
