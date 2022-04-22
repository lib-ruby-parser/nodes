use crate::messages_data;
use crate::nodes_data;

pub struct LiquidTemplate {
    path: String,
    filters: Vec<Box<dyn liquid_core::parser::ParseFilter>>,
    globals: liquid::Object,
}

impl LiquidTemplate {
    pub fn new<P: AsRef<str>>(path: P) -> Self {
        let path = path.as_ref().to_string();
        Self {
            path: path.clone(),
            filters: crate::filters::all(),
            globals: liquid::object!({
                "nodes": nodes_data::ALL_NODES,
                "messages": messages_data::ALL_MESSAGES,
                "template": path
            }),
        }
    }

    pub fn with_filter<F>(mut self, f: F) -> Self
    where
        F: Into<Box<dyn liquid_core::parser::ParseFilter>>,
    {
        self.filters.push(f.into());
        self
    }

    pub fn with_global(mut self, name: &str, value: liquid_core::Value) -> Self {
        self.globals.insert(name.to_owned().into(), value);
        self
    }

    pub fn render(self) -> String {
        let Self {
            path,
            filters,
            globals,
        } = self;

        println!("cargo:rerun-if-changed={}", path);

        let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read {}:\n{}", path, e);
            std::process::exit(1);
        });

        let mut builder = liquid::ParserBuilder::with_stdlib();
        for filter in filters {
            builder = builder.filter(filter);
        }

        let parser = builder.build().unwrap_or_else(|e| {
            eprintln!("Failed to construct parser:\n{}", e);
            std::process::exit(1);
        });

        let template = parser.parse(&src).unwrap_or_else(|e| {
            eprintln!("Liquid template error ({}):\n{}", path, e);
            std::process::exit(1);
        });

        template.render(&globals).unwrap_or_else(|e| {
            eprintln!("Failed to render {} template:\n{}", path, e);
            std::process::exit(1);
        })
    }
}
