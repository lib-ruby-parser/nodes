use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "snakecase_to_camelcase",
    description = "Renders comment with prefix/offset params.",
    parsed(SnakecaseToCamelcaseFilter)
)]
pub struct SnakecaseToCamelcase;

#[derive(Debug, Default, Display_filter)]
#[name = "snakecase_to_camelcase"]
struct SnakecaseToCamelcaseFilter;

impl Filter for SnakecaseToCamelcaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_kstr();
        let output = crate::helpers::snakecase_to_camelcase(&input);
        Ok(Value::scalar(output))
    }
}
