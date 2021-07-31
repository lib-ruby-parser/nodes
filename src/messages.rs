use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SectionList(pub Vec<Section>);

impl SectionList {
    pub(crate) fn new(sections: Vec<Section>) -> Self {
        Self(sections)
    }

    pub fn map(&self, f: &dyn Fn(&Section) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&Section) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }

    pub fn messages(&self) -> MessageList {
        let messages: Vec<Message> = self.0.iter().flat_map(|s| s.messages.0.clone()).collect();
        MessageList(messages)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Section {
    pub name: String,
    pub messages: MessageList,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageList(pub Vec<Message>);

impl MessageList {
    pub fn map(&self, f: &dyn Fn(&Message) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&Message) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Message {
    pub name: String,
    pub fields: MessageFieldList,
    pub comment: String,
}

impl Message {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(&self.comment, prefix).to_string(offset)
    }

    pub fn camelcase_name(&self) -> String {
        self.name.to_string()
    }

    pub fn upper_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(&self.camelcase_name()).to_uppercase()
    }

    pub fn lower_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(&self.camelcase_name()).to_lowercase()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageFieldList(pub Vec<MessageField>);

impl MessageFieldList {
    pub fn map(&self, f: &dyn Fn(&MessageField) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&MessageField) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageField {
    pub name: String,
    pub field_type: MessageFieldType,
    pub comment: String,
}

impl MessageField {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(&self.comment, prefix).to_string(offset)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum MessageFieldType {
    Str,
    Byte,
}
