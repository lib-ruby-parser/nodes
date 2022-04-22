use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Node {
    pub camelcase_name: &'static str,
    pub wqp_name: &'static str,
    pub fields: &'static [&'static NodeField],
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
