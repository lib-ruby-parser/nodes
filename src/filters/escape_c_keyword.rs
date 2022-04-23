use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "escape_c_keyword",
    description = "Escape C keyword by adding trailing _",
    parsed(EscapeCKeywordFilter)
)]
pub struct EscapeCKeyword;

#[derive(Debug, Default, Display_filter)]
#[name = "escape_c_keyword"]
struct EscapeCKeywordFilter;

impl Filter for EscapeCKeywordFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let name = input.to_kstr();
        let output = crate::helpers::escape_c_keyword(&name);
        Ok(Value::scalar(output))
    }
}
