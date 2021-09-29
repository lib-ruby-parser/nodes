use crate::template::structs::{
    loops_bounds, string_breakers, MessageFieldTemplate, MessageFieldTemplatePart, MessageTemplate,
    MessageTemplatePart, NodeFieldTemplate, NodeFieldTemplatePart, NodeTemplate, NodeTemplatePart,
    Template, TemplatePart,
};
use crate::template::{
    shards::{Condition, Helper, Loop, LoopBounds, StringPart, StringPartBreakers},
    Buffer, ParseError,
};

pub(crate) trait Parse {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError>
    where
        Self: Sized;
}

// Loop bounds implementations
impl LoopBounds for loops_bounds::NodesLoopBounds {
    const START: &'static str = "<each-node>";
    const END: &'static str = "</each-node>";
}
impl LoopBounds for loops_bounds::NodeFieldsLoopBounds {
    const START: &'static str = "<each-node-field>";
    const END: &'static str = "</each-node-field>";
}
impl LoopBounds for loops_bounds::MessagesLoopBounds {
    const START: &'static str = "<each-message>";
    const END: &'static str = "</each-message>";
}
impl LoopBounds for loops_bounds::MessageFieldsLoopBounds {
    const START: &'static str = "<each-message-field>";
    const END: &'static str = "</each-message-field>";
}

// String breakers implementation
impl StringPartBreakers for string_breakers::TemplateStringBreakers {
    const BREAKERS: &'static [&'static str] = &[
        Helper::START,
        loops_bounds::NodesLoopBounds::START,
        loops_bounds::MessagesLoopBounds::START,
        Condition::<Template>::START,
    ];
}
impl StringPartBreakers for string_breakers::NodeTemplateStringBreakers {
    const BREAKERS: &'static [&'static str] = &[
        Helper::START,
        loops_bounds::NodeFieldsLoopBounds::START,
        Condition::<NodeTemplate>::START,
    ];
}
impl StringPartBreakers for string_breakers::NodeFieldTemplateStringBreakers {
    const BREAKERS: &'static [&'static str] =
        &[Helper::START, Condition::<NodeFieldTemplate>::START];
}
impl StringPartBreakers for string_breakers::MessageTemplateStringBreakers {
    const BREAKERS: &'static [&'static str] = &[
        Helper::START,
        Condition::<MessageTemplate>::START,
        loops_bounds::MessageFieldsLoopBounds::START,
    ];
}
impl StringPartBreakers for string_breakers::MessageFieldTemplateStringBreakers {
    const BREAKERS: &'static [&'static str] =
        &[Helper::START, Condition::<MessageFieldTemplate>::START];
}

impl Parse for TemplatePart {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        if buffer.is(Helper::START) {
            // global helper
            let helper = Helper::parse(buffer)?;
            return Ok(Self::GlobalHelper(helper));
        }

        if buffer.is(loops_bounds::NodesLoopBounds::START) {
            // loop over nodes
            let loop_ = Loop::<NodeTemplate, loops_bounds::NodesLoopBounds>::parse(buffer)?;
            return Ok(Self::NodesLoop(loop_));
        }

        if buffer.is(loops_bounds::MessagesLoopBounds::START) {
            // loop over messages
            let loop_ = Loop::<MessageTemplate, loops_bounds::MessagesLoopBounds>::parse(buffer)?;
            return Ok(Self::MessagesLoop(loop_));
        }

        if buffer.is(Condition::<Template>::START) {
            // global condition
            let condition = Condition::<Template>::parse(buffer)?;
            return Ok(Self::GlobalCondition(condition));
        }

        // fallback to string
        let string = StringPart::<string_breakers::TemplateStringBreakers>::parse(buffer)?;
        Ok(Self::StringPart(string))
    }
}

impl Parse for NodeTemplatePart {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        if buffer.is(Helper::START) {
            // node helper
            let helper = Helper::parse(buffer)?;
            return Ok(Self::Helper(helper));
        }

        if buffer.is(loops_bounds::NodeFieldsLoopBounds::START) {
            // loop over node fields
            let loop_ =
                Loop::<NodeFieldTemplate, loops_bounds::NodeFieldsLoopBounds>::parse(buffer)?;
            return Ok(Self::FieldsLoop(loop_));
        }

        if buffer.is(Condition::<NodeTemplate>::START) {
            // node condition
            let condition = Condition::<NodeTemplate>::parse(buffer)?;
            return Ok(Self::Condition(condition));
        }

        // fallback to string
        let string = StringPart::<string_breakers::NodeTemplateStringBreakers>::parse(buffer)?;
        Ok(Self::StringPart(string))
    }
}

impl Parse for NodeFieldTemplatePart {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        if buffer.is(Helper::START) {
            // field helper
            let helper = Helper::parse(buffer)?;
            return Ok(Self::Helper(helper));
        }

        if buffer.is(Condition::<NodeFieldTemplate>::START) {
            // condition
            let condition = Condition::<NodeFieldTemplate>::parse(buffer)?;
            return Ok(Self::Condition(condition));
        }

        // fallback to string
        let string = StringPart::<string_breakers::NodeFieldTemplateStringBreakers>::parse(buffer)?;
        Ok(Self::StringPart(string))
    }
}

impl Parse for MessageTemplatePart {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        if buffer.is(Helper::START) {
            // message helper
            let helper = Helper::parse(buffer)?;
            return Ok(Self::Helper(helper));
        }

        if buffer.is(Condition::<MessageTemplate>::START) {
            // condition
            let condition = Condition::<MessageTemplate>::parse(buffer)?;
            return Ok(Self::Condition(condition));
        }

        if buffer.is(loops_bounds::MessageFieldsLoopBounds::START) {
            // loop over message fields
            let loop_ =
                Loop::<MessageFieldTemplate, loops_bounds::MessageFieldsLoopBounds>::parse(buffer)?;
            return Ok(Self::FieldsLoop(loop_));
        }

        // fallback to string
        let string = StringPart::<string_breakers::MessageTemplateStringBreakers>::parse(buffer)?;
        Ok(Self::StringPart(string))
    }
}

impl Parse for MessageFieldTemplatePart {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        if buffer.is(Helper::START) {
            // field helper
            let helper = Helper::parse(buffer)?;
            return Ok(Self::Helper(helper));
        }

        if buffer.is(Condition::<MessageFieldTemplate>::START) {
            // condition
            let condition = Condition::<MessageFieldTemplate>::parse(buffer)?;
            return Ok(Self::Condition(condition));
        }

        // fallback to string
        let string =
            StringPart::<string_breakers::MessageFieldTemplateStringBreakers>::parse(buffer)?;
        Ok(Self::StringPart(string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEMPLATE: &str = "<helper codegen-info>

<each-node>
There is a node <helper node-name>
It has fields:
    <each-node-field>
        + <helper node-field-name> (printable: <if is-always-print>YES<else>NO</if>)
    </each-node-field>
</each-node>

<each-message>
There is a message <helper message-name>
It has fields:
    <each-message-field>
        + <helper message-field-name> (foo: <if cond>A<else>B</if>)
    </each-message-field>
</each-message>
";

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new(TEMPLATE.as_bytes().to_vec());
        let parsed = Template::parse(&mut buffer).unwrap();

        println!("{:#?}", parsed);

        assert_eq!(
            parsed,
            Template::new([
                TemplatePart::GlobalHelper(Helper::new("codegen-info")),
                TemplatePart::StringPart(StringPart::new("\n\n")),
                TemplatePart::NodesLoop(Loop::new(NodeTemplate::new([
                    NodeTemplatePart::StringPart(StringPart::new("\nThere is a node ")),
                    NodeTemplatePart::Helper(Helper::new("node-name")),
                    NodeTemplatePart::StringPart(StringPart::new("\nIt has fields:\n    ")),
                    NodeTemplatePart::FieldsLoop(Loop::new(NodeFieldTemplate::new([
                        NodeFieldTemplatePart::StringPart(StringPart::new("\n        + ")),
                        NodeFieldTemplatePart::Helper(Helper::new("node-field-name")),
                        NodeFieldTemplatePart::StringPart(StringPart::new(" (printable: ")),
                        NodeFieldTemplatePart::Condition(Condition::new(
                            String::from("is-always-print"),
                            NodeFieldTemplate::new([NodeFieldTemplatePart::StringPart(
                                StringPart::new("YES")
                            )]),
                            NodeFieldTemplate::new([NodeFieldTemplatePart::StringPart(
                                StringPart::new("NO")
                            )]),
                        )),
                        NodeFieldTemplatePart::StringPart(StringPart::new(")\n    ")),
                    ]))),
                    NodeTemplatePart::StringPart(StringPart::new("\n")),
                ]))),
                TemplatePart::StringPart(StringPart::new("\n\n")),
                // 2nd loop
                TemplatePart::MessagesLoop(Loop::new(MessageTemplate::new([
                    MessageTemplatePart::StringPart(StringPart::new("\nThere is a message ")),
                    MessageTemplatePart::Helper(Helper::new("message-name")),
                    MessageTemplatePart::StringPart(StringPart::new("\nIt has fields:\n    ")),
                    MessageTemplatePart::FieldsLoop(Loop::new(MessageFieldTemplate::new([
                        MessageFieldTemplatePart::StringPart(StringPart::new("\n        + ")),
                        MessageFieldTemplatePart::Helper(Helper::new("message-field-name")),
                        MessageFieldTemplatePart::StringPart(StringPart::new(" (foo: ")),
                        MessageFieldTemplatePart::Condition(Condition::new(
                            "cond",
                            MessageFieldTemplate::new([MessageFieldTemplatePart::StringPart(
                                StringPart::new("A")
                            )]),
                            MessageFieldTemplate::new([MessageFieldTemplatePart::StringPart(
                                StringPart::new("B")
                            )]),
                        )),
                        MessageFieldTemplatePart::StringPart(StringPart::new(")\n    ")),
                    ]))),
                    MessageTemplatePart::StringPart(StringPart::new("\n")),
                ],))),
                TemplatePart::StringPart(StringPart::new("\n")),
            ])
        )
    }
}
