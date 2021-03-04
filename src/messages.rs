use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub name: String,
    pub fields: Vec<MessageField>,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageField {
    pub name: String,
    pub field_type: MessageFieldType,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MessageFieldType {
    Str,
    Byte,
    Bytes,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Messages {
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Section {
    pub name: String,
    pub messages: Vec<Message>,
}
