use lib_ruby_parser_nodes::{messages, nodes, Message, MessageField, Node, NodeField};

fn main() {
    render_nodes();
    render_messages();
}

fn render_nodes() {
    let contents = nodes()
        .iter()
        .map(|node| render_node(*node))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write("NODES.md", contents).unwrap()
}

fn render_node(node: &Node) -> String {
    format!(
        "### {node_name}

{comment}

Fields:

{fields}
",
        node_name = node.camelcase_name,
        comment = node.render_comment("", 3),
        fields = node
            .fields
            .iter()
            .enumerate()
            .map(|(idx, node_field)| render_node_field(idx + 1, *node_field))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn render_node_field(idx: usize, node_field: &NodeField) -> String {
    format!(
        "{idx}. **{field_name}** (`{field_type}`)

{comment}
",
        idx = idx,
        field_name = node_field.snakecase_name,
        field_type = format!("{:?}", node_field.field_type),
        comment = node_field.render_comment("", 3)
    )
}

fn render_messages() {
    let contents = messages()
        .iter()
        .map(|message| render_message(*message))
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write("MESSAGES.md", contents).unwrap()
}

fn render_message(message: &Message) -> String {
    format!(
        "### {message_name}

{comment}

Fields:

{fields}
",
        message_name = message.camelcase_name,
        comment = message.render_comment("", 3),
        fields = message
            .fields
            .iter()
            .enumerate()
            .map(|(idx, message_field)| render_message_field(idx + 1, *message_field))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn render_message_field(idx: usize, message_field: &MessageField) -> String {
    format!(
        "{idx}. **{field_name}** (`{field_type}`)

{comment}
",
        idx = idx,
        field_name = message_field.snakecase_name,
        field_type = format!("{:?}", message_field.field_type),
        comment = message_field.render_comment("", 3)
    )
}
