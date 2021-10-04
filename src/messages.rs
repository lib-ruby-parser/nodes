#[derive(Debug, Clone)]
pub struct MessagesList(pub &'static [Message]);

impl MessagesList {
    pub fn map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&Message) -> String,
    {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&Message) -> Vec<String>,
    {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub camelcase_name: &'static str,
    pub fields: MessageFieldList,
    pub comment: &'static [&'static str],
}

impl Message {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(self.comment, prefix).to_string(offset)
    }

    pub fn upper_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(self.camelcase_name).to_uppercase()
    }

    pub fn lower_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(self.camelcase_name).to_lowercase()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageFieldList(pub &'static [MessageField]);

impl MessageFieldList {
    pub fn map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&MessageField) -> String,
    {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&MessageField) -> Vec<String>,
    {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageField {
    pub camelcase_name: &'static str,
    pub field_type: MessageFieldType,
    pub comment: &'static [&'static str],
}

impl MessageField {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(self.comment, prefix).to_string(offset)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageFieldType {
    Str,
    Byte,
}
