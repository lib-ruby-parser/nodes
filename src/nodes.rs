use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct NodeList(pub Vec<Node>);

impl NodeList {
    pub(crate) fn new(nodes: Vec<Node>) -> Self {
        Self(nodes)
    }

    pub fn map(&self, f: &dyn Fn(&Node) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&Node) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Node {
    pub struct_name: String,
    pub str_type: String,
    pub filename: String,
    pub fields: NodeFieldList,
    pub comment: String,
}

impl Node {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(&self.comment, prefix).to_string(offset)
    }

    pub fn camelcase_name(&self) -> String {
        self.struct_name.to_string()
    }

    pub fn upper_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(&self.camelcase_name()).to_uppercase()
    }

    pub fn lower_name(&self) -> String {
        crate::helpers::camel_case_to_underscored(&self.camelcase_name()).to_lowercase()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct NodeFieldList(pub Vec<NodeField>);

impl NodeFieldList {
    pub fn any_field_has_type(&self, field_type: NodeFieldType) -> bool {
        self.0.iter().any(|f| f.field_type == field_type)
    }

    pub fn map(&self, f: &dyn Fn(&NodeField) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&NodeField) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NodeField {
    pub field_name: String,
    pub field_type: NodeFieldType,
    pub always_print: bool,
    pub comment: String,
}

impl NodeField {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(&self.comment, prefix).to_string(offset)
    }
}

#[derive(PartialEq, Clone, Deserialize, Debug)]
pub enum NodeFieldType {
    Node,
    Nodes,
    MaybeNode,
    Loc,
    MaybeLoc,
    Str,
    MaybeStr,
    Chars,
    StringValue,
    U8,
    Usize,
    RawString,
    RegexOptions,
}

impl NodeFieldType {}
