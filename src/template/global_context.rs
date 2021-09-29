use crate::{Message, Node};

#[derive(Debug, PartialEq)]
pub struct GlobalContext {
    pub nodes: &'static [Node],
    pub messages: &'static [Message],
}

pub const ALL_DATA: &GlobalContext = &GlobalContext {
    nodes: crate::nodes_data::ALL_NODES.0,
    messages: crate::messages_data::ALL_MESSAGES.0,
};
