extern crate liquid;
extern crate serde;

pub mod filters;
pub mod helpers;
mod messages;
#[allow(non_upper_case_globals)]
mod messages_data;
mod nodes;
#[allow(non_upper_case_globals)]
mod nodes_data;

pub use messages::*;
pub use nodes::*;

pub fn nodes() -> NodeList {
    nodes_data::ALL_NODES
}

pub fn messages() -> MessagesList {
    messages_data::ALL_MESSAGES
}

mod liquid_template;
pub use liquid_template::LiquidTemplate;

pub mod reexports {
    pub mod liquid {
        pub use liquid_core::value;
    }
    pub mod serde {
        pub use serde::{ser::SerializeStruct, Serialize, Serializer};
    }
}

#[cfg(test)]
mod tests {
    use crate::NodeFieldType;

    use super::{messages, nodes};

    #[test]
    fn test_nodes() {
        let nodes = nodes();
        assert!(nodes.len() > 0);
    }

    #[test]
    fn test_nodes_order() {
        let nodes = nodes();

        let contents = std::fs::read_to_string("src/nodes_data.rs").unwrap();
        let mut node_decls = vec![];
        for line in contents.lines() {
            if line.starts_with("static") && line.ends_with(": Node = Node {") {
                let decl = line
                    .strip_prefix("static ")
                    .unwrap()
                    .strip_suffix(": Node = Node {")
                    .unwrap();
                node_decls.push(decl);
            }
        }
        for (left, right) in node_decls.iter().zip(node_decls.iter().skip(1)) {
            assert!(
                left.to_lowercase() < right.to_lowercase(),
                "node declarations are not sorted: {} goes before {}",
                left,
                right
            )
        }

        for (left, right) in nodes.iter().zip(nodes.iter().skip(1)) {
            let lhs = left.camelcase_name;
            let rhs = right.camelcase_name;
            assert!(
                lhs.to_lowercase() < rhs.to_lowercase(),
                "nodes are not sorted: {} goes before {}",
                lhs,
                rhs
            )
        }

        assert_eq!(node_decls.len(), nodes.len());

        for (node_decl, node) in node_decls.iter().zip(nodes.iter()) {
            assert_eq!(
                node_decl, &node.camelcase_name,
                "node declaration order doesn't match nodes order: {} / {}",
                node_decl, node.camelcase_name
            )
        }
    }

    #[test]
    fn test_node_fields() {
        let nodes = nodes();

        for node in nodes {
            assert!(
                node.fields
                    .iter()
                    .any(|f| f.snakecase_name == "expression_l"),
                "node {} doesn't have 'expression_l' field",
                node.camelcase_name
            );

            let mut found_loc = false;
            for node_field in node.fields {
                if node_field.field_type == NodeFieldType::Loc
                    || node_field.field_type == NodeFieldType::MaybeLoc
                {
                    found_loc = true;
                } else if found_loc {
                    panic!(
                        "Fields of node {} are not sorted: {} goes after *_l fields",
                        node.camelcase_name, node_field.snakecase_name
                    )
                }
            }
        }
    }

    #[test]
    fn test_messages() {
        let messages = messages();
        assert!(messages.len() > 0);
    }
}
