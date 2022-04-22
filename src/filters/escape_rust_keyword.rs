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
        let output = crate::helpers::escape_rust_keyword(&name);
        Ok(Value::scalar(output))
    }
}
