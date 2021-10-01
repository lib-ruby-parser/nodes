use crate::template::{Buffer, Parse, ParseError, ParseErrorKind, Render};

pub(crate) trait LoopBounds {
    const START: &'static str;
    const END: &'static str;
}

#[derive(Debug, PartialEq)]
pub(crate) struct Loop<Body, Bounds> {
    body: Body,
    phantom: std::marker::PhantomData<Bounds>,
}

impl<Body, Bounds> Loop<Body, Bounds> {
    pub(crate) fn new(body: Body) -> Self {
        Self {
            body,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<Body, Bounds> Parse for Loop<Body, Bounds>
where
    Bounds: LoopBounds,
    Body: Parse,
{
    fn parse(buffer: &mut Buffer) -> Result<Self, ParseError> {
        // skip "<each-?>", it's there
        buffer
            .take(Bounds::START.len())
            .expect("bug: Loop::START is not in the buffer");

        // capture loop body
        let body = buffer
            .take_until_pattern(Bounds::END)
            .ok_or_else(|| ParseError {
                kind: ParseErrorKind::MissingLoopBody,
                pos: buffer.pos(),
            })?;

        // skip "</each-?>"
        if buffer.is(Bounds::END) {
            buffer
                .take(Bounds::END.len())
                .expect("bug: Loop::END is not in the buffer");
        } else {
            return Err(ParseError {
                kind: ParseErrorKind::MissingLoopClosingTag,
                pos: buffer.pos(),
            });
        }

        let mut buffer = Buffer::new(body.into_bytes());
        let body = Body::parse(&mut buffer)?;

        Ok(Self::new(body))
    }
}

impl<Body, Bounds, Context> Render<[Context]> for Loop<Body, Bounds>
where
    Body: Render<Context>,
{
    fn render(&self, ctxs: &[Context], fns: &crate::template::TemplateFns) -> String {
        ctxs.iter()
            .map(|ctx| self.body.render(ctx, fns))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{
        shards::Char,
        shards::StringPart,
        structs::{Template, TemplatePart},
        TemplateFns,
    };

    #[derive(Debug, PartialEq)]
    struct DummyLoopBounds;
    impl LoopBounds for DummyLoopBounds {
        const START: &'static str = "<dummy-loop-start>";
        const END: &'static str = "<dummy-loop-end>";
    }

    #[test]
    fn test_parse() {
        let template = "<dummy-loop-start>BODY<dummy-loop-end>";
        let mut buffer = Buffer::new(template.as_bytes().to_vec());
        let parsed = Loop::<Template, DummyLoopBounds>::parse(&mut buffer).unwrap();

        assert_eq!(
            parsed,
            Loop::new(Template::new([TemplatePart::StringPart(StringPart::new(
                "BODY"
            ))]))
        )
    }

    #[test]
    fn test_render() {
        let loop_ = Loop::<Char, DummyLoopBounds>::new(Char { c: 'a' });
        let ctx = vec!['a', 'b', 'c'];
        let fns = TemplateFns::new();
        assert_eq!(
            "provided char a\nprovided char b\nprovided char c\n",
            loop_.render(&ctx, &fns)
        )
    }
}
