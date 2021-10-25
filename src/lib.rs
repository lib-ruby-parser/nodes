pub mod comment;
pub mod helpers;
mod messages;
#[allow(non_upper_case_globals)]
mod messages_data;
mod nodes;
#[allow(non_upper_case_globals)]
mod nodes_data;
pub mod template;

pub use messages::*;
pub use nodes::*;

pub fn nodes() -> NodeList {
    nodes_data::ALL_NODES
}

pub fn messages() -> MessagesList {
    messages_data::ALL_MESSAGES
}

#[cfg(test)]
mod tests {
    use super::{messages, nodes};

    #[test]
    fn test_nodes() {
        assert!(nodes().len() > 0);
        for node in nodes() {
            for node_field in node.fields {
                let lhs = node_field.node;
                let rhs = *node;
                assert_eq!(lhs.camelcase_name, rhs.camelcase_name);
            }
        }
    }

    #[test]
    fn test_messages() {
        assert!(messages().len() > 0)
    }
}
