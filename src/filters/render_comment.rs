use crate::filters::invalid_input;
use liquid_core::Expression;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Value, ValueView};

#[derive(Debug, FilterParameters)]
struct RenderCommentArgs {
    #[parameter(description = "Line prefix.", arg_type = "str")]
    prefix: Option<Expression>,

    #[parameter(description = "Line offset.", arg_type = "integer")]
    offset: Option<Expression>,
}

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "render_comment",
    description = "Renders comment with prefix/offset params.",
    parameters(RenderCommentArgs),
    parsed(RenderCommentFilter)
)]
pub struct RenderComment;

#[derive(Debug, FromFilterParameters, Display_filter)]
#[name = "render_comment"]
struct RenderCommentFilter {
    #[parameters]
    args: RenderCommentArgs,
}

impl Filter for RenderCommentFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;

        let prefix = args
            .prefix
            .ok_or_else(|| invalid_input("prefix argument is required"))?;

        let offset = args
            .offset
            .ok_or_else(|| invalid_input("offset argument is required"))?;

        let input = input
            .as_array()
            .ok_or_else(|| invalid_input("Array of strings expected"))?;

        let lines = input.values().map(|x| x.to_kstr());

        let rendered = lines
            .enumerate()
            .map(|(idx, line)| {
                let mut line = line.to_string();
                if !line.is_empty() {
                    line = format!(" {}", line);
                }
                let spaces = if idx == 0 {
                    // Do not add prefix to the first line
                    String::from("")
                } else {
                    " ".repeat(offset as usize)
                };
                format!(
                    "{spaces}{prefix}{line}",
                    spaces = spaces,
                    prefix = prefix,
                    line = line
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(Value::scalar(rendered))
    }
}
