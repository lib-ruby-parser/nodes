use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "camelcase_to_snakecase",
    description = "Renders comment with prefix/offset params.",
    parsed(CamelcaseToSnakecaseFilter)
)]
pub struct CamelcaseToSnakecase;

#[derive(Debug, Default, Display_filter)]
#[name = "camelcase_to_snakecase"]
struct CamelcaseToSnakecaseFilter;

impl Filter for CamelcaseToSnakecaseFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_kstr();

        let mut words = vec![];
        let mut word = String::from("");

        for c in input.chars() {
            if c.is_uppercase() {
                // flush
                words.push(word);
                word = String::from("");
            }
            word.push(c);
        }

        words.push(word);

        let result = words
            .into_iter()
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>()
            .join("_")
            .to_lowercase();

        Ok(Value::scalar(result))
    }
}
