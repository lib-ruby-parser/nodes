use crate::template::fns::BucketKey;
use crate::template::{render::Render, shards::FnName, Buffer, Dispatch, Parse, TemplateFns, F};

#[derive(Debug, PartialEq)]
pub struct Condition<Branch> {
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

        // capture if-false body
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
    Context: BucketKey,
    TemplateFns: Dispatch<Context, F::Predicate>,
{
    fn render(&self, ctx: &Context, fns: &TemplateFns) -> String {
        let predicate_value = fns
            .dispatch(&self.predicate_name, ctx)
            .unwrap_or_else(|| panic!("Can't find node predicate {}", self.predicate_name));

        let branch = if predicate_value {
            &self.if_true
        } else {
            &self.if_false
        };

        if let Some(branch) = branch.as_ref() {
            branch.render(ctx, fns)
        } else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::template::{
        global_context::NO_DATA,
        shards::StringPart,
        structs::{Template, TemplatePart},
        GlobalContext, F,
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
        fns.register::<GlobalContext, F::Predicate>("foo", always_true);
        assert_eq!("1", condition.render(NO_DATA, &fns));

        let mut fns = TemplateFns::new();
        fn always_false(_: &GlobalContext) -> bool {
            false
        }
        fns.register::<GlobalContext, F::Predicate>("foo", always_false);
        assert_eq!("2", condition.render(NO_DATA, &fns));
    }
}
