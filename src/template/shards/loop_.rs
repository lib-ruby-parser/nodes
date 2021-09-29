use crate::template::{Buffer, Parse, ParseError, ParseErrorKind, Render};

pub(crate) trait LoopBounds {
    const START: &'static str;
    const END: &'static str;
}

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

impl<Body, Bounds> std::fmt::Debug for Loop<Body, Bounds>
where
    Body: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Loop").field("body", &self.body).finish()
    }
}

impl<Body, Bounds> PartialEq for Loop<Body, Bounds>
where
    Body: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.body == other.body
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
