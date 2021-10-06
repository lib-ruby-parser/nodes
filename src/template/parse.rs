use crate::template::structs::{
    MessageFieldTemplate, MessageFieldTemplatePart, MessageTemplate, MessageTemplatePart,
    NodeFieldTemplate, NodeFieldTemplatePart, NodeTemplate, NodeTemplatePart, Template,
    TemplatePart,
};
use crate::template::{
    shards::{Condition, Helper, Loop, LoopBody, StringPart},
    Buffer,
};

pub(crate) trait Parse {
    fn parse(buffer: &mut Buffer) -> Option<Self>
    where
        Self: Sized;
}

impl Parse for TemplatePart {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        if let Some(helper) = Helper::parse(buffer) {
            return Some(Self::GlobalHelper(helper));
        }

        if let Some(loop_) = Loop::parse(buffer) {
            return Some(Self::NodesLoop(loop_));
        }

        if let Some(loop_) = Loop::parse(buffer) {
            return Some(Self::MessagesLoop(loop_));
        }

        if let Some(condition) = Condition::<Template>::parse(buffer) {
            return Some(Self::GlobalCondition(condition));
        }

        if let Some(string) = StringPart::parse(buffer) {
            return Some(Self::StringPart(string));
        }

        None
    }
}

impl LoopBody for NodeTemplate {
    const KIND: &'static str = "node";
}

impl Parse for NodeTemplatePart {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        if let Some(helper) = Helper::parse(buffer) {
            return Some(Self::Helper(helper));
        }

        if let Some(loop_) = Loop::parse(buffer) {
            return Some(Self::FieldsLoop(loop_));
        }

        if let Some(condition) = Condition::<NodeTemplate>::parse(buffer) {
            return Some(Self::Condition(condition));
        }

        if let Some(string) = StringPart::parse(buffer) {
            return Some(Self::StringPart(string));
        }

        None
    }
}

impl LoopBody for NodeFieldTemplate {
    const KIND: &'static str = "node-field";
}

impl Parse for NodeFieldTemplatePart {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        if let Some(helper) = Helper::parse(buffer) {
            return Some(Self::Helper(helper));
        }

        if let Some(condition) = Condition::<NodeFieldTemplate>::parse(buffer) {
            return Some(Self::Condition(condition));
        }

        if let Some(string) = StringPart::parse(buffer) {
            return Some(Self::StringPart(string));
        }

        None
    }
}

impl LoopBody for MessageTemplate {
    const KIND: &'static str = "message";
}

impl Parse for MessageTemplatePart {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        if let Some(helper) = Helper::parse(buffer) {
            return Some(Self::Helper(helper));
        }

        if let Some(condition) = Condition::<MessageTemplate>::parse(buffer) {
            return Some(Self::Condition(condition));
        }

        if let Some(loop_) = Loop::parse(buffer) {
            return Some(Self::FieldsLoop(loop_));
        }

        if let Some(string) = StringPart::parse(buffer) {
            return Some(Self::StringPart(string));
        }

        None
    }
}

impl LoopBody for MessageFieldTemplate {
    const KIND: &'static str = "message-field";
}

impl Parse for MessageFieldTemplatePart {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        if let Some(helper) = Helper::parse(buffer) {
            return Some(Self::Helper(helper));
        }

        if let Some(condition) = Condition::<MessageFieldTemplate>::parse(buffer) {
            return Some(Self::Condition(condition));
        }

        if let Some(string) = StringPart::parse(buffer) {
            return Some(Self::StringPart(string));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEMPLATE: &str = "{{ helper codegen-info }}

{{ each node }}
There is a node {{ helper node-name }}
It has fields:
    {{ each node-field }}
        + {{ helper node-field-name }} (printable: {{ if is-always-print }}YES{{ else }}NO{{ end }})
    {{ end }}
{{ end }}

{{ each message }}
There is a message {{ helper message-name }}
It has fields:
    {{ each message-field }}
        + {{ helper message-field-name }} (foo: {{ if cond }}A{{ else }}B{{ end }})
    {{ end }}
{{ end }}
";

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new(TEMPLATE.as_bytes().to_vec());
        let parsed = Template::parse(&mut buffer).unwrap();

        assert_eq!(
            parsed,
            Template::new([
                TemplatePart::GlobalHelper(Helper::new("codegen-info")),
                TemplatePart::StringPart(StringPart::new("\n\n")),
                TemplatePart::NodesLoop(Loop::new(Some(NodeTemplate::new([
                    NodeTemplatePart::StringPart(StringPart::new("\nThere is a node ")),
                    NodeTemplatePart::Helper(Helper::new("node-name")),
                    NodeTemplatePart::StringPart(StringPart::new("\nIt has fields:\n    ")),
                    NodeTemplatePart::FieldsLoop(Loop::new(Some(NodeFieldTemplate::new([
                        NodeFieldTemplatePart::StringPart(StringPart::new("\n        + ")),
                        NodeFieldTemplatePart::Helper(Helper::new("node-field-name")),
                        NodeFieldTemplatePart::StringPart(StringPart::new(" (printable: ")),
                        NodeFieldTemplatePart::Condition(Condition::new(
                            String::from("is-always-print"),
                            Some(NodeFieldTemplate::new([NodeFieldTemplatePart::StringPart(
                                StringPart::new("YES")
                            )])),
                            Some(NodeFieldTemplate::new([NodeFieldTemplatePart::StringPart(
                                StringPart::new("NO")
                            )])),
                        )),
                        NodeFieldTemplatePart::StringPart(StringPart::new(")\n    ")),
                    ])))),
                    NodeTemplatePart::StringPart(StringPart::new("\n")),
                ])))),
                TemplatePart::StringPart(StringPart::new("\n\n")),
                // 2nd loop
                TemplatePart::MessagesLoop(Loop::new(Some(MessageTemplate::new([
                    MessageTemplatePart::StringPart(StringPart::new("\nThere is a message ")),
                    MessageTemplatePart::Helper(Helper::new("message-name")),
                    MessageTemplatePart::StringPart(StringPart::new("\nIt has fields:\n    ")),
                    MessageTemplatePart::FieldsLoop(Loop::new(Some(MessageFieldTemplate::new([
                        MessageFieldTemplatePart::StringPart(StringPart::new("\n        + ")),
                        MessageFieldTemplatePart::Helper(Helper::new("message-field-name")),
                        MessageFieldTemplatePart::StringPart(StringPart::new(" (foo: ")),
                        MessageFieldTemplatePart::Condition(Condition::new(
                            "cond",
                            Some(MessageFieldTemplate::new([
                                MessageFieldTemplatePart::StringPart(StringPart::new("A"))
                            ])),
                            Some(MessageFieldTemplate::new([
                                MessageFieldTemplatePart::StringPart(StringPart::new("B"))
                            ])),
                        )),
                        MessageFieldTemplatePart::StringPart(StringPart::new(")\n    ")),
                    ])))),
                    MessageTemplatePart::StringPart(StringPart::new("\n")),
                ])))),
                TemplatePart::StringPart(StringPart::new("\n")),
            ])
        )
    }
}
