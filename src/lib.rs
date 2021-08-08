pub mod comment;
pub mod helpers;
mod messages;
mod messages_data;
mod nodes;
mod nodes_data;

pub use messages::*;
pub use nodes::*;

pub fn nodes() -> NodeList {
    nodes_data::ALL_NODES
}

pub fn message_sections() -> SectionList {
    messages_data::ALL_SECTIONS
}

pub fn messages() -> DynamicMessageList {
    message_sections().messages()
}

#[cfg(test)]
mod tests {
    use super::{message_sections, messages, nodes};

    #[test]
    fn test_nodes() {
        assert!(nodes().0.len() > 0);
    }

    #[test]
    fn test_message_sections() {
        let message_sections = message_sections();
        assert!(message_sections.0.len() > 0);
        for section in message_sections.0.iter() {
            assert!(section.messages.0.len() > 0);
        }
    }

    #[test]
    fn test_messages() {
        assert!(messages().0.len() > 0)
    }
}
