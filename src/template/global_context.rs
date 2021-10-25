use crate::{Message, Node};

#[derive(Debug, PartialEq)]
pub struct GlobalContext {
    pub nodes: &'static [&'static Node],
    pub messages: &'static [&'static Message],
}

pub static ALL_DATA: &GlobalContext = &GlobalContext {
    nodes: crate::nodes_data::ALL_NODES,
    messages: crate::messages_data::ALL_MESSAGES,
};

#[cfg(test)]
pub(crate) const NO_DATA: &GlobalContext = &GlobalContext {
    nodes: &[],
    messages: &[],
};
