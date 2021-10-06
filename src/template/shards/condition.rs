use crate::template::fns::FnSubject;
use crate::template::{render::Render, shards::FnName, Buffer, Parse, TemplateFns};

#[derive(Debug, PartialEq)]
pub(crate) struct Condition<Branch> {
    predicate_name: String,
    if_true: Option<Branch>,
    if_false: Option<Branch>,
}

impl<Branch> Condition<Branch> {
    pub(crate) fn new<S>(
        predicate_name: S,
        if_true: Option<Branch>,
        if_false: Option<Branch>,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            predicate_name: predicate_name.into(),
            if_true,
            if_false,
        }
    }
}

impl<Branch> Parse for Condition<Branch>
where
    Branch: Parse,
{
    fn parse(buffer: &mut Buffer) -> Option<Self> {
        // consume "{{ if "
        if !buffer.is("{{ if ") {
            return None;
        }
        buffer.consume("{{ if ");

        // capture predicate
        let start = buffer.pos();
        let predicate_name = FnName::parse(buffer)
            .unwrap_or_else(|| panic!("predicate name is empty at pos {}", start))
            .unwrap();

        // consume " }}"
        if !buffer.is(" }}") {
            panic!("{{ if }} is not closed at {}", buffer.pos());
        }
        buffer.consume(" }}");

        // capture if-true body
        let if_true = Branch::parse(buffer);

        // consume "{{ else }}"
        if !buffer.is("{{ else }}") {
            panic!("expected to get {{ else }} at {}", buffer.pos())
        }
        buffer.consume("{{ else }}");

        // capture if-true body
        let if_false = Branch::parse(buffer);

        // consume "{{ end }}"
        if !buffer.is("{{ end }}") {
            panic!("expected to get {{ end }} at {}", buffer.pos())
        }
        buffer.consume("{{ end }}");

        Some(Self::new(predicate_name, if_true, if_false))
    }
}

impl<Context, Branch> Render<Context> for Condition<Branch>
where
    Branch: Render<Context>,
    Context: FnSubject,
{
    fn render(&self, ctx: &Context, fns: &TemplateFns) -> String {
        let predicate_value = ctx
            .dispatch_predicate(fns, &self.predicate_name)
            .unwrap_or_else(|| panic!("Can't find node predicate {}", self.predicate_name));
        if predicate_value {
            if let Some(if_true) = &self.if_true {
                return if_true.render(ctx, fns);
            }
        } else {
            if let Some(if_false) = &self.if_false {
                return if_false.render(ctx, fns);
            }
        }
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use crate::template::{
        global_context::NO_DATA,
        shards::StringPart,
        structs::{Template, TemplatePart},
        GlobalContext,
    };

    use super::*;

    #[test]
    fn test_parse() {
        let mut buffer = Buffer::new("{{ if foo }}1{{ else }}2{{ end }}".as_bytes().to_vec());
        let parsed = Condition::<Template>::parse(&mut buffer).unwrap();

        assert_eq!(
            parsed,
            Condition::new(
                "foo",
                Some(Template::new([TemplatePart::StringPart(StringPart::new(
                    "1"
                ))])),
                Some(Template::new([TemplatePart::StringPart(StringPart::new(
                    "2"
                ))]))
            )
        )
    }

    #[test]
    fn test_render() {
        let condition = Condition::new(
            "foo",
            Some(Template::new([TemplatePart::StringPart(StringPart::new(
                "1",
            ))])),
            Some(Template::new([TemplatePart::StringPart(StringPart::new(
                "2",
            ))])),
        );

        let mut fns = TemplateFns::new();
        fn always_true(_: &GlobalContext) -> bool {
            true
        }
        fns.register_predicate("foo", always_true);
        assert_eq!("1", condition.render(NO_DATA, &fns));

        let mut fns = TemplateFns::new();
        fn always_false(_: &GlobalContext) -> bool {
            false
        }
        fns.register_predicate("foo", always_false);
        assert_eq!("2", condition.render(NO_DATA, &fns));
    }
}
