pub type MessagesList = &'static [&'static Message];

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

pub type MessageFieldList = &'static [&'static MessageField];

#[derive(Debug, Clone, PartialEq)]
pub struct MessageField {
    pub message: &'static Message,
    pub snakecase_name: &'static str,
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

#[macro_export]
macro_rules! message_has_field {
    ($message:ident, $( $pattern:pat_param )|+) => {
        $message.fields.iter().any(|f| matches!(f.field_type, $( $pattern )|+))
    };
}

pub use message_has_field;

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MESSAGE: Message = Message {
        camelcase_name: "",
        fields: &[&MessageField {
            message: &TEST_MESSAGE,
            snakecase_name: "",
            field_type: MessageFieldType::Byte,
            comment: &[],
        }],
        comment: &[],
    };

    #[test]
    fn test_message_has_field() {
        assert!(message_has_field!(TEST_MESSAGE, MessageFieldType::Byte));
        assert!(!message_has_field!(TEST_MESSAGE, MessageFieldType::Str))
    }
}
