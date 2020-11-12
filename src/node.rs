use serde::Deserialize;

use crate::{Field, Options};

#[derive(Deserialize)]
pub struct Node {
    pub struct_name: String,
    pub str_type: String,
    pub filename: String,
    pub fields: Vec<Field>,
    pub comment: Option<String>,
}

impl Node {
    pub fn uses(&self, options: &Options) -> Vec<String> {
        let uses = &options.uses;
        uses(self)
    }

    pub fn fields_declaration(&self, options: &Options) -> String {
        self.fields
            .iter()
            .map(|f| f.declaration(options))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn comment(&self) -> String {
        self.comment
            .clone()
            .unwrap_or_else(|| String::from(""))
            .lines()
            .map(|s| format!("/// {}", s).trim().to_owned())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn code(&self, options: &Options) -> String {
        let epilogue = &options.epilogue;

        format!(
            "{uses}
{comment}
#[derive(Debug, Clone, PartialEq)]
pub struct {struct_name} {{
{declare_fields}
}}

{epilogue}
",
            uses = self.uses(options).join("\n"),
            comment = self.comment(),
            struct_name = self.struct_name,
            declare_fields = self.fields_declaration(options),
            epilogue = epilogue(self)
        )
    }
}
