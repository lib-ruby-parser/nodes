#[derive(Debug, Clone)]
pub struct SectionList(pub &'static [Section]);

impl SectionList {
    pub fn map(&self, f: &dyn Fn(&Section) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&Section) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }

    pub fn messages(&self) -> DynamicMessageList {
        let messages: Vec<Message> = self.0.iter().flat_map(|s| s.messages.0.to_vec()).collect();
        DynamicMessageList(messages)
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: &'static str,
    pub messages: BuiltinMessageList,
}

#[derive(Debug, Clone)]
pub struct BuiltinMessageList(pub &'static [Message]);

impl BuiltinMessageList {
    pub fn map(&self, f: &dyn Fn(&Message) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&Message) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone)]
pub struct DynamicMessageList(pub Vec<Message>);

impl DynamicMessageList {
    pub fn map(&self, f: &dyn Fn(&Message) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&Message) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub camelcase_name: &'static str,
    pub fields: MessageFieldList,
    pub comment: &'static [&'static str],
}

impl Message {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(&self.comment, prefix).to_string(offset)
    }

    pub fn upper_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(self.camelcase_name).to_uppercase()
    }

    pub fn lower_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(self.camelcase_name).to_lowercase()
    }
}

#[derive(Debug, Clone)]
pub struct MessageFieldList(pub &'static [MessageField]);

impl MessageFieldList {
    pub fn map(&self, f: &dyn Fn(&MessageField) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&MessageField) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone)]
pub struct MessageField {
    pub name: &'static str,
    pub field_type: MessageFieldType,
    pub comment: &'static [&'static str],
}

impl MessageField {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(&self.comment, prefix).to_string(offset)
    }
}

#[derive(Debug, Clone)]
pub enum MessageFieldType {
    Str,
    Byte,
}
