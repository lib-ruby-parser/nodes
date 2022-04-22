use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Message {
    pub camelcase_name: &'static str,
    pub fields: &'static [&'static MessageField],
    pub comment: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct MessageField {
    pub snakecase_name: &'static str,
    pub field_type: MessageFieldType,
    pub comment: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum MessageFieldType {
    Str,
    Byte,
}
