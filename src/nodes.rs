pub type NodeList = &'static [&'static Node];

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub camelcase_name: &'static str,
    pub wqp_name: &'static str,
    pub fields: NodeFieldList,
    pub comment: &'static [&'static str],
}

impl Node {
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

pub type NodeFieldList = &'static [&'static NodeField];

#[derive(Debug, Clone)]
pub struct NodeField {
    pub node: &'static Node,
    pub snakecase_name: &'static str,
    pub field_type: NodeFieldType,
    pub always_print: bool,
    pub comment: &'static [&'static str],
}

impl PartialEq for NodeField {
    fn eq(&self, other: &Self) -> bool {
        self.snakecase_name == other.snakecase_name
            && self.field_type == other.field_type
            && self.always_print == other.always_print
            && self.comment == other.comment
    }
}

impl NodeField {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(self.comment, prefix).to_string(offset)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum NodeFieldType {
    Node,
    Nodes,
    MaybeNode { regexp_options: bool },
    Loc,
    MaybeLoc,
    Str { raw: bool },
    MaybeStr { chars: bool },
    StringValue,
    U8,
}

impl NodeFieldType {}

#[macro_export]
macro_rules! node_has_field {
    ($node:ident, $( $pattern:pat_param )|+) => {
        $node.fields.iter().any(|f| matches!(f.field_type, $( $pattern )|+))
    };
}

pub use node_has_field;

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_NODE: Node = Node {
        camelcase_name: "",
        wqp_name: "",
        fields: &[&NodeField {
            node: &TEST_NODE,
            snakecase_name: "",
            field_type: NodeFieldType::Loc,
            always_print: true,
            comment: &[],
        }],
        comment: &[],
    };

    #[test]
    fn test_node_has_field() {
        assert!(node_has_field!(TEST_NODE, NodeFieldType::Loc));
        assert!(!node_has_field!(TEST_NODE, NodeFieldType::MaybeNode { .. }))
    }
}
