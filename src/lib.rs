extern crate serde;
extern crate serde_yaml;

use std::path::Path;

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

pub fn nodes() -> Vec<Node> {
    let nodes_yaml = yaml_file("nodes.yaml");
    serde_yaml::from_reader(nodes_yaml).unwrap()
}

pub fn messages() -> Messages {
    let messages_yaml = yaml_file("messages.yaml");
    serde_yaml::from_reader(messages_yaml).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{messages, nodes};

    #[test]
    fn test_nodes() {
        assert!(nodes().len() > 0)
    }

    #[test]
    fn test_messages() {
        let messages = messages();
        assert!(messages.sections.len() > 0);
        for section in messages.sections.iter() {
            assert!(section.messages.len() > 0);
        }
        println!("{:#?}", messages);
    }
}
