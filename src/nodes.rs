#[derive(Debug, Clone)]
pub struct NodeList(pub &'static [Node]);

impl NodeList {
    pub fn map(&self, f: &dyn Fn(&Node) -> String) -> Vec<String> {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map(&self, f: &dyn Fn(&Node) -> Vec<String>) -> Vec<String> {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub struct_name: &'static str,
    pub str_type: &'static str,
    pub filename: &'static str,
    pub fields: NodeFieldList,
    pub comment: &'static [&'static str],
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

#[derive(Debug, Clone)]
pub struct NodeFieldList(pub &'static [NodeField]);

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

#[derive(Debug, Clone)]
pub struct NodeField {
    pub field_name: &'static str,
    pub field_type: NodeFieldType,
    pub always_print: bool,
    pub comment: &'static [&'static str],
}

impl NodeField {
    pub fn render_comment(&self, prefix: &str, offset: usize) -> String {
        crate::comment::Comment::new(&self.comment, prefix).to_string(offset)
    }
}

#[derive(PartialEq, Clone, Debug)]
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
