#[derive(Debug)]
pub enum ParseErrorKind {
    // <each-node>...</each-node>
    // <each-field>...</each-field>
    MissingLoopClosingTag,
    MissingLoopBody,

    // <helper ...>
    MissingHelperName,
    MissingHelperClose,

    // <if ...> ... <else> ... </if>
    MissingPredicateInCondition,
    MissingIfTrueBody,
    MissingElse,
    MissingIfFalseBody,
    MissingIfClosingTag,

    UnexpectedEof,
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub pos: usize,
}
