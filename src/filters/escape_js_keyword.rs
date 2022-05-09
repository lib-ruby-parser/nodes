use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "escape_js_keyword",
    description = "Escape Js keyword by adding trailing _",
    parsed(EscapeJsKeywordFilter)
)]
pub struct EscapeJsKeyword;

#[derive(Debug, Default, Display_filter)]
#[name = "escape_js_keyword"]
struct EscapeJsKeywordFilter;

impl Filter for EscapeJsKeywordFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let name = input.to_kstr();
        let output = crate::helpers::escape_js_keyword(&name);
        Ok(Value::scalar(output))
    }
}
