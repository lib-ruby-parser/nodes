use serde::Serialize;

pub type MessagesList = &'static [&'static Message];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Message {
    pub camelcase_name: &'static str,
    pub fields: MessageFieldList,
    pub comment: &'static [&'static str],
}

impl Message {
    pub fn upper_name(&self) -> String {
        crate::helpers::camelcase_to_snakecase(self.camelcase_name).to_uppercase()
    }

    pub fn lower_name(&self) -> String {
        crate::helpers::camelcase_to_snakecase(self.camelcase_name).to_lowercase()
    }
}

pub type MessageFieldList = &'static [&'static MessageField];

#[derive(Debug, Clone, Eq, PartialOrd, Ord, Serialize)]
pub struct MessageField {
    pub snakecase_name: &'static str,
    pub field_type: MessageFieldType,
    pub comment: &'static [&'static str],
}

impl PartialEq for MessageField {
    fn eq(&self, other: &Self) -> bool {
        self.snakecase_name == other.snakecase_name
            && self.field_type == other.field_type
            && self.comment == other.comment
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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

    const TEST_MESSAGE: Message = Message {
        camelcase_name: "",
        fields: &[&MessageField {
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
