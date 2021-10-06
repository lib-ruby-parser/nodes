use crate::template::shards::{Condition, Helper, List, Loop, StringPart};

pub(crate) type Template = List<TemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum TemplatePart {
    StringPart(StringPart),
    GlobalHelper(Helper),
    GlobalCondition(Condition<Template>),
    NodesLoop(Loop<NodeTemplate>),
    MessagesLoop(Loop<MessageTemplate>),
}

pub(crate) type NodeTemplate = List<NodeTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum NodeTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<NodeTemplate>),
    FieldsLoop(Loop<NodeFieldTemplate>),
}

pub(crate) type NodeFieldTemplate = List<NodeFieldTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum NodeFieldTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<NodeFieldTemplate>),
}

pub(crate) type MessageTemplate = List<MessageTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum MessageTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<MessageTemplate>),
    FieldsLoop(Loop<MessageFieldTemplate>),
}

pub(crate) type MessageFieldTemplate = List<MessageFieldTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum MessageFieldTemplatePart {
    StringPart(StringPart),
    Helper(Helper),
    Condition(Condition<MessageFieldTemplate>),
}
