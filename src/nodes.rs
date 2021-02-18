use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Node {
    pub struct_name: String,
    pub str_type: String,
    pub filename: String,
    pub fields: Vec<Field>,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Field {
    pub field_name: String,
    pub field_type: FieldType,
    pub always_print: bool,
    pub comment: String,
}

#[derive(PartialEq, Clone, Deserialize, Debug)]
pub enum FieldType {
    Node,
    Nodes,
    MaybeNode,
    Loc,
    MaybeLoc,
    Str,
    MaybeStr,
    Chars,
    StringValue,
    U8,
    Usize,
    RawString,
    RegexOptions,
}

impl FieldType {
    pub fn has_reference_to_node(&self) -> bool {
        matches!(
            self,
            Self::Node | Self::Nodes | Self::MaybeNode | Self::RegexOptions
        )
    }

    pub fn has_reference_to_loc(&self) -> bool {
        matches!(self, FieldType::Loc | FieldType::MaybeLoc)
    }
}
