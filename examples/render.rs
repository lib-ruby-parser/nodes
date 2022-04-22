use lib_ruby_parser_nodes::LiquidTemplate;

mod filters {
    use lib_ruby_parser_nodes::filters::invalid_input;
    use liquid_core::Result;
    use liquid_core::Runtime;
    use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
    use liquid_core::{Value, ValueView};

    #[derive(Clone, ParseFilter, FilterReflection)]
    #[filter(
        name = "dbg",
        description = "Renders Debug representation.",
        parsed(DbgFilter)
    )]
    pub struct Dbg;

    #[derive(Debug, Default, Display_filter)]
    #[name = "dbg"]
    struct DbgFilter;

    fn node_field_from_liquid_object(input: &dyn ValueView) -> Result<String> {
        if let Some(object) = input.as_object() {
            let variant = object.keys().next().unwrap();
            let fields = object.get(&variant).unwrap().as_object().unwrap();
            let mut output = format!("{} {{ ", variant);
            for (key, value) in fields.iter() {
                let value = value.render();
                output.push_str(&format!("{}: {}", key, value));
            }
            output.push_str(" }");

            Ok(output)
        } else if let Some(scalar) = input.as_scalar() {
            Ok(scalar.into_string().to_string())
        } else {
            Err(invalid_input("argument must be either an object or scalar"))
        }
    }

    impl Filter for DbgFilter {
        fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
            Ok(Value::scalar(node_field_from_liquid_object(input)?))
        }
    }
}

fn main() {
    let nodes = LiquidTemplate::new("examples/nodes.liquid")
        .with_filter(filters::Dbg)
        .with_global("numbers", liquid_core::value!(vec![1, 2, 3]))
        .render();
    std::fs::write("NODES.md", nodes).unwrap();

    let messages = LiquidTemplate::new("examples/messages.liquid").render();
    std::fs::write("MESSAGES.md", messages).unwrap();
}
