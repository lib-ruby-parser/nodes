use serde::Deserialize;

use crate::FieldType;
use crate::Options;

#[derive(Clone, Deserialize)]
pub struct Field {
    pub field_name: String,
    pub field_type: FieldType,
    pub always_print: bool,
    pub comment: Option<String>,
}

impl Field {
    pub fn declaration(&self, options: &Options) -> String {
        let map_field = &options.map_field;
        let field_type = map_field(&self.field_type);

        let offset = "    ";
        format!(
            "{comments}\n{offset}pub {field_name}: {field_type},",
            offset = offset,
            comments = self
                .comment
                .clone()
                .unwrap_or_else(|| String::from(""))
                .lines()
                .map(|s| format!("{}/// {}", offset, s).trim_end().to_owned())
                .collect::<Vec<_>>()
                .join("\n"),
            field_name = self.field_name,
            field_type = field_type
        )
    }
}
