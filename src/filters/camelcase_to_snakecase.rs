use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "camelcase_to_snakecase",
    description = "Convert CamelCase to snake_case.",
    parsed(CamelcaseToSnakecaseFilter)
)]
pub struct CamelcaseToSnakecase;

#[derive(Debug, Default, Display_filter)]
#[name = "camelcase_to_snakecase"]
struct CamelcaseToSnakecaseFilter;

impl Filter for CamelcaseToSnakecaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_kstr();
        let output = crate::helpers::camelcase_to_snakecase(&input);
        Ok(Value::scalar(output))
    }
}
