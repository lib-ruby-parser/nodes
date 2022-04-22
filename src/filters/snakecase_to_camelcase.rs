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

        fn capitalize_word(s: &str) -> String {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        }

        let result = input.split('_').map(capitalize_word).collect::<String>();

        Ok(Value::scalar(result))
    }
}
