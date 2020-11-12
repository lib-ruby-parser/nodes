use crate::{FieldType, Node};

pub struct Options {
    pub target_dir: String,
    pub map_field: Box<dyn Fn(&FieldType) -> String>,
    pub uses: Box<dyn Fn(&Node) -> Vec<String>>,
    pub epilogue: Box<dyn Fn(&Node) -> String>,
}
