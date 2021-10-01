use crate::template::{Buffer, GlobalContext, Parse, ParseError, Render, TemplateFns};

// Dummy strategy for parsing,
// Hidden behind `#[cfg(test)]`
#[derive(Debug, PartialEq)]
pub(crate) struct Char {
    pub(crate) c: char,
}

impl Render<GlobalContext> for Char {
    fn render(&self, _ctx: &GlobalContext, _fns: &TemplateFns) -> String {
        format!("stored char {}\n", self.c)
    }
}

impl Render<char> for Char {
    fn render(&self, c: &char, _fns: &TemplateFns) -> String {
        format!("provided char {}\n", c)
    }
}

impl Parse for Char {
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        let c = buffer.take(1).unwrap().chars().next().unwrap();
        Ok(Self { c })
    }
}
