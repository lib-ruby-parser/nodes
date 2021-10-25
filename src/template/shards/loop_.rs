use crate::template::{Buffer, Parse, Render, TemplateFns};

pub trait LoopBody {
    const KIND: &'static str;
}

#[derive(Debug, PartialEq)]
pub struct Loop<Body> {
    body: Option<Body>,
}

impl<Body> Loop<Body> {
    pub(crate) fn new(body: Option<Body>) -> Self {
        Self { body }
    }
}

impl<Body> Parse for Loop<Body>
where
    Body: Parse + LoopBody,
{
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        // consume "{{ each KIND }}"
        let loop_start = format!("{{{{ each {} }}}}", Body::KIND);
        if !buffer.is(&loop_start) {
            return None;
        }
        buffer.consume(&loop_start);

        // capture loop body
        let body = Body::parse(buffer);

        // consume "{{ end }}"
        if !buffer.is("{{ end }}") {
            panic!("{} is not closed at {}", loop_start, buffer.pos());
        }
        buffer.consume("{{ end }}");

        Some(Self::new(body))
    }
}

impl<Body, Context> Render<[&Context]> for Loop<Body>
where
    Body: Render<Context>,
{
    fn render(&self, ctxs: &[&Context], fns: &TemplateFns) -> String {
        if let Some(body) = &self.body {
            ctxs.iter()
                .map(|ctx| body.render(ctx, fns))
                .collect::<Vec<_>>()
                .join("")
        } else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{
        shards::Char,
        shards::StringPart,
        structs::{NodeTemplate, NodeTemplatePart},
        TemplateFns,
    };

    #[test]
    fn test_parse() {
        let template = "{{ each node }}BODY{{ end }}";
        let mut buffer = Buffer::new(template.as_bytes().to_vec());
        let parsed = Loop::<NodeTemplate>::parse(&mut buffer).unwrap();

        assert_eq!(
            parsed,
            Loop::new(Some(NodeTemplate::new([NodeTemplatePart::StringPart(
                StringPart::new("BODY")
            )])))
        )
    }

    #[test]
    fn test_render() {
        let loop_ = Loop::<Char>::new(Some(Char { c: 'a' }));
        let ctx = vec![&'a', &'b', &'c'];
        let fns = TemplateFns::new();
        assert_eq!(
            "provided char a\nprovided char b\nprovided char c\n",
            loop_.render(&ctx, &fns)
        )
    }
}
