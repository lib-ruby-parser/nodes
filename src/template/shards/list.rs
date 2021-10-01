use crate::template::{render::Render, Buffer, Parse, ParseError, TemplateFns};

pub struct List<Item> {
    parts: Vec<Item>,
}

impl<Item> List<Item> {
    pub(crate) fn new<V>(parts: V) -> Self
    where
        V: Into<Vec<Item>>,
    {
        Self {
            parts: parts.into(),
        }
    }
}

impl<Item> std::fmt::Debug for List<Item>
where
    Item: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List").field("parts", &self.parts).finish()
    }
}

impl<Item> PartialEq for List<Item>
where
    Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.parts == other.parts
    }
}

impl<Item> Parse for List<Item>
where
    Item: Parse,
{
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        let mut parts = vec![];
        while !buffer.is_eof() {
            let part = Item::parse(buffer)?;
            parts.push(part)
        }
        Ok(Self::new(parts))
    }
}

impl<Item, Context> Render<Context> for List<Item>
where
    Item: Render<Context>,
{
    fn render(&self, ctx: &Context, fns: &TemplateFns) -> String {
        self.parts
            .iter()
            .map(|item| item.render(ctx, fns))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{global_context::NO_DATA, shards::Char};

    type CharList = List<Char>;

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new("abc".as_bytes().to_vec());
        let parsed = CharList::parse(&mut buffer).unwrap();

        assert_eq!(
            parsed,
            CharList::new([Char { c: 'a' }, Char { c: 'b' }, Char { c: 'c' }])
        )
    }

    #[test]
    fn test_render() {
        let list = CharList::new([Char { c: 'a' }, Char { c: 'b' }, Char { c: 'c' }]);
        let fns = TemplateFns::new();
        assert_eq!(
            "stored char a\nstored char b\nstored char c\n",
            list.render(NO_DATA, &fns)
        )
    }
}
