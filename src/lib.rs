extern crate serde;
extern crate serde_yaml;

use std::path::Path;

pub mod comment;
pub mod helpers;
mod messages;
mod nodes;

pub use messages::*;
pub use nodes::*;

fn yaml_file(yaml: &str) -> std::fs::File {
    let path = Path::new(file!())
        .parent()
        .unwrap()
        .join(yaml)
        .to_str()
        .unwrap()
        .to_owned();

    std::fs::File::open(&path).unwrap()
}

pub fn nodes() -> NodeList {
    let nodes_yaml = yaml_file("nodes.yaml");
    let nodes: Vec<Node> = serde_yaml::from_reader(nodes_yaml).unwrap();
    NodeList::new(nodes)
}

pub fn message_sections() -> SectionList {
    let messages_yaml = yaml_file("messages.yaml");
    let sections: Vec<Section> = serde_yaml::from_reader(messages_yaml).unwrap();
    SectionList::new(sections)
}

pub fn messages() -> MessageList {
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
