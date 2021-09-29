use crate::template::shards::{Condition, Helper, List, Loop, StringPart};

pub(crate) type Template = List<TemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum TemplatePart {
    StringPart(StringPart<string_breakers::TemplateStringBreakers>),
    GlobalHelper(Helper),
    GlobalCondition(Condition<Template>),
    NodesLoop(Loop<NodeTemplate, loops_bounds::NodesLoopBounds>),
    MessagesLoop(Loop<MessageTemplate, loops_bounds::MessagesLoopBounds>),
}

pub(crate) type NodeTemplate = List<NodeTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum NodeTemplatePart {
    StringPart(StringPart<string_breakers::NodeTemplateStringBreakers>),
    Helper(Helper),
    Condition(Condition<NodeTemplate>),
    FieldsLoop(Loop<NodeFieldTemplate, loops_bounds::NodeFieldsLoopBounds>),
}

pub(crate) type NodeFieldTemplate = List<NodeFieldTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum NodeFieldTemplatePart {
    StringPart(StringPart<string_breakers::NodeFieldTemplateStringBreakers>),
    Helper(Helper),
    Condition(Condition<NodeFieldTemplate>),
}

pub(crate) type MessageTemplate = List<MessageTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum MessageTemplatePart {
    StringPart(StringPart<string_breakers::MessageTemplateStringBreakers>),
    Helper(Helper),
    Condition(Condition<MessageTemplate>),
    FieldsLoop(Loop<MessageFieldTemplate, loops_bounds::MessageFieldsLoopBounds>),
}

pub(crate) type MessageFieldTemplate = List<MessageFieldTemplatePart>;

#[derive(Debug, PartialEq)]
pub(crate) enum MessageFieldTemplatePart {
    StringPart(StringPart<string_breakers::MessageFieldTemplateStringBreakers>),
    Helper(Helper),
    Condition(Condition<MessageFieldTemplate>),
}

pub(crate) mod loops_bounds {
    pub(crate) struct NodesLoopBounds;
    pub(crate) struct NodeFieldsLoopBounds;
    pub(crate) struct MessagesLoopBounds;
    pub(crate) struct MessageFieldsLoopBounds;
}

pub(crate) mod string_breakers {
    #[derive(Debug, PartialEq)]
    pub(crate) struct TemplateStringBreakers;
    #[derive(Debug, PartialEq)]
    pub(crate) struct NodeTemplateStringBreakers;
    #[derive(Debug, PartialEq)]
    pub(crate) struct NodeFieldTemplateStringBreakers;
    #[derive(Debug, PartialEq)]
    pub(crate) struct MessageTemplateStringBreakers;
    #[derive(Debug, PartialEq)]
    pub(crate) struct MessageFieldTemplateStringBreakers;
}
