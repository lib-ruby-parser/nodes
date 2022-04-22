use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "escape_rust_keyword",
    description = "Escape Rust keyword by adding trailing _",
    parsed(EscapeRustKeywordFilter)
)]
pub struct EscapeRustKeyword;

#[derive(Debug, Default, Display_filter)]
#[name = "escape_rust_keyword"]
struct EscapeRustKeywordFilter;

impl Filter for EscapeRustKeywordFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let name = input.to_kstr();
        let escaped = match &name[..] {
            "const" | "as" | "else" | "self" | "break" | "false" | "for" | "if" | "return"
            | "str" | "super" | "true" | "while" | "yield" => format!("{}_", name),
            other => other.to_string(),
        };

        Ok(Value::scalar(escaped))
    }
}
