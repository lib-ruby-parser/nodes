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

#[derive(Debug, Clone, PartialEq)]
pub struct NodeField {
    pub node: &'static Node,
    pub snakecase_name: &'static str,
    pub field_type: NodeFieldType,
    pub always_print: bool,
    pub comment: &'static [&'static str],
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
