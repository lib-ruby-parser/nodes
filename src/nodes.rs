#[derive(Debug, Clone)]
pub struct NodeList(pub &'static [Node]);

impl NodeList {
    pub fn map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&Node) -> String,
    {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&Node) -> Vec<String>,
    {
        self.0.iter().flat_map(f).collect()
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct NodeFieldList(pub &'static [NodeField]);

impl NodeFieldList {
    pub fn any_field_has_type(&self, field_type: NodeFieldType) -> bool {
        self.0.iter().any(|f| f.field_type == field_type)
    }

    pub fn map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&NodeField) -> String,
    {
        self.0.iter().map(f).collect()
    }

    pub fn flat_map<F>(&self, f: F) -> Vec<String>
    where
        F: Fn(&NodeField) -> Vec<String>,
    {
        self.0.iter().flat_map(f).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeField {
    pub camelcase_name: &'static str,
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
