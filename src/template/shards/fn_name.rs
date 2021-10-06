use crate::template::{Buffer, Parse};

pub(crate) struct FnName {
    name: String,
}

impl FnName {
    pub(crate) fn unwrap(self) -> String {
        self.name
    }
}

impl Parse for FnName {
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        let mut name = String::from("");
        while !buffer.is_eof() {
            let c = buffer.take_char().expect("bug: unexpected EOF");

            if c.is_alphanumeric() || c == '_' || c == '-' {
                name.push(c)
            } else {
                buffer.set_pos(buffer.pos() - 1);
                break;
            }
        }

        if name.is_empty() {
            None
        } else {
            Some(Self { name })
        }
    }
}
