pub mod comment;
pub mod helpers;
mod messages;
mod messages_data;
mod nodes;
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
        assert!(nodes().0.len() > 0);
    }

    #[test]
    fn test_messages() {
        assert!(messages().0.len() > 0)
    }
}
