use serde::Deserialize;

#[derive(PartialEq, Clone, Deserialize, Debug)]
pub enum FieldType {
    Node,
    Nodes,
    MaybeNode,
    Range,
    MaybeRange,
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
        match self {
            FieldType::Node => true,
            FieldType::Nodes => true,
            FieldType::MaybeNode => true,
            FieldType::Range => false,
            FieldType::MaybeRange => false,
            FieldType::Str => false,
            FieldType::MaybeStr => false,
            FieldType::Chars => false,
            FieldType::StringValue => false,
            FieldType::U8 => false,
            FieldType::Usize => false,
            FieldType::RawString => false,
            FieldType::RegexOptions => true,
        }
    }

    pub fn has_reference_to_range(&self) -> bool {
        match self {
            FieldType::Node => false,
            FieldType::Nodes => false,
            FieldType::MaybeNode => false,
            FieldType::Range => true,
            FieldType::MaybeRange => true,
            FieldType::Str => false,
            FieldType::MaybeStr => false,
            FieldType::Chars => false,
            FieldType::StringValue => false,
            FieldType::U8 => false,
            FieldType::Usize => false,
            FieldType::RawString => false,
            FieldType::RegexOptions => false,
        }
    }
}
