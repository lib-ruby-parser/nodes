use crate::template::shards::{Condition, Helper, List, Loop, StringPart};

pub type Template = List<TemplatePart>;

#[derive(Debug, PartialEq)]
pub enum TemplatePart {
    StringPart(StringPart),
    GlobalHelper(Helper),
    GlobalCondition(Condition<Template>),
    NodesLoop(Loop<NodeTemplate>),
    MessagesLoop(Loop<MessageTemplate>),
}

pub type NodeTemplate = List<NodeTemplatePart>;

#[derive(Debug, PartialEq)]
pub enum NodeTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<NodeTemplate>),
    FieldsLoop(Loop<NodeFieldTemplate>),
}

pub type NodeFieldTemplate = List<NodeFieldTemplatePart>;

#[derive(Debug, PartialEq)]
pub enum NodeFieldTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<NodeFieldTemplate>),
}

pub type MessageTemplate = List<MessageTemplatePart>;

#[derive(Debug, PartialEq)]
pub enum MessageTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<MessageTemplate>),
    FieldsLoop(Loop<MessageFieldTemplate>),
}

pub type MessageFieldTemplate = List<MessageFieldTemplatePart>;

#[derive(Debug, PartialEq)]
pub enum MessageFieldTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<MessageFieldTemplate>),
}
