use serde::Serialize;

pub type NodeList = &'static [&'static Node];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Node {
    pub camelcase_name: &'static str,
    pub wqp_name: &'static str,
    pub fields: NodeFieldList,
    pub comment: &'static [&'static str],
}

impl Node {
    pub fn upper_name(&self) -> String {
        crate::helpers::camelcase_to_snakecase(self.camelcase_name).to_uppercase()
    }

    pub fn lower_name(&self) -> String {
        crate::helpers::camelcase_to_snakecase(self.camelcase_name).to_lowercase()
    }
}

pub type NodeFieldList = &'static [&'static NodeField];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct NodeField {
    pub snakecase_name: &'static str,
    pub field_type: NodeFieldType,
    pub always_print: bool,
    pub comment: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum NodeFieldType {
    Node,
    Nodes,
    MaybeNode,
    RegexpOptions,
    Loc,
    MaybeLoc,
    Str,
    RawStr,
    MaybeStr,
    Chars,
    StringValue,
    U8,
}

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
