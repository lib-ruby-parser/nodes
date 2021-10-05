use crate::template::GlobalContext;
use crate::{Message, MessageWithField, Node, NodeWithField};
use std::collections::HashMap;

pub trait PerTypeFnRegistry {
    type Subject;

    fn register_helper(&mut self, helper: &str, f: fn(&Self::Subject) -> String);
    fn get_helper(&self, helper: &str) -> fn(&Self::Subject) -> String;

    fn register_predicate(&mut self, predicate: &str, f: fn(&Self::Subject) -> bool);
    fn get_predicate(&self, predicate: &str) -> fn(&Self::Subject) -> bool;
}

pub struct Bucket<T> {
    helpers: HashMap<String, fn(&T) -> String>,
    predicates: HashMap<String, fn(&T) -> bool>,
}

impl<T> Default for Bucket<T> {
    fn default() -> Self {
        Self {
            helpers: HashMap::default(),
            predicates: HashMap::default(),
        }
    }
}

impl<T> Bucket<T> {
    pub(crate) fn register_helper(&mut self, helper: &str, f: fn(&T) -> String) {
        self.helpers.insert(helper.to_string(), f);
    }

    pub(crate) fn get_helper(&self, helper: &str) -> Option<fn(&T) -> String> {
        self.helpers.get(helper).copied()
    }

    pub(crate) fn register_predicate(&mut self, predicate: &str, f: fn(&T) -> bool) {
        self.predicates.insert(predicate.to_string(), f);
    }

    pub(crate) fn get_predicate(&self, predicate: &str) -> Option<fn(&T) -> bool> {
        self.predicates.get(predicate).copied()
    }
}

#[derive(Default)]
pub struct Fns {
    global: Bucket<GlobalContext>,
    node: Bucket<Node>,
    node_field: Bucket<NodeWithField>,
    message: Bucket<Message>,
    message_field: Bucket<MessageWithField>,
}

impl Fns {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_helper<T>(&mut self, helper: &str, f: fn(&T) -> String)
    where
        T: FnSubject,
    {
        T::get_mut(self).register_helper(helper, f);
    }

    pub fn register_predicate<T>(&mut self, predicate: &str, f: fn(&T) -> bool)
    where
        T: FnSubject,
    {
        T::get_mut(self).register_predicate(predicate, f);
    }
}

pub trait FnSubject {
    fn get(fns: &Fns) -> &Bucket<Self>
    where
        Self: Sized;
    fn get_mut(fns: &mut Fns) -> &mut Bucket<Self>
    where
        Self: Sized;
}

macro_rules! def_impl {
    ($t:ty, $bucket_name:ident) => {
        impl FnSubject for $t {
            fn get(fns: &Fns) -> &Bucket<Self> {
                &fns.$bucket_name
            }

            fn get_mut(fns: &mut Fns) -> &mut Bucket<Self> {
                &mut fns.$bucket_name
            }
        }
    };
}
def_impl!(GlobalContext, global);
def_impl!(Node, node);
def_impl!(NodeWithField, node_field);
def_impl!(Message, message);
def_impl!(MessageWithField, message_field);

pub(crate) trait GetRegistrySlice<T> {
    fn get_slice(&self) -> &Bucket<T>;
    fn get_slice_mut(&mut self) -> &mut Bucket<T>;
}

impl<T> GetRegistrySlice<T> for Fns
where
    T: FnSubject,
{
    fn get_slice(&self) -> &Bucket<T> {
        T::get(self)
    }

    fn get_slice_mut(&mut self) -> &mut Bucket<T> {
        T::get_mut(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{MessageFieldList, NodeFieldList};

    use super::*;

    #[test]
    fn test_helper() {
        let mut fns = Fns::new();
        fn node_helper(node: &Node) -> String {
            format!("from node_helper {}", node.camelcase_name)
        }
        let node = Node {
            camelcase_name: "Foo",
            wqp_name: "foo",
            fields: NodeFieldList(&[]),
            comment: &[],
        };
        fns.register_helper("node-helper", node_helper);
        let bucket: &Bucket<Node> = fns.get_slice();
        let helper = bucket.get_helper("node-helper").unwrap();
        assert_eq!(helper(&node), node_helper(&node));
    }

    #[test]
    fn test_predicate() {
        let mut fns = Fns::new();
        fn message_predicate(message: &Message) -> bool {
            message.camelcase_name == "Bar"
        }
        let message_t = Message {
            camelcase_name: "Bar",
            fields: MessageFieldList(&[]),
            comment: &[],
        };
        let message_f = Message {
            camelcase_name: "NotBar",
            fields: MessageFieldList(&[]),
            comment: &[],
        };
        fns.register_predicate("message-predicate", message_predicate);
        let bucket: &Bucket<Message> = fns.get_slice();
        let predicate = bucket.get_predicate("message-predicate").unwrap();
        assert_eq!(predicate(&message_t), message_predicate(&message_t));
        assert_eq!(predicate(&message_f), message_predicate(&message_f));
    }

    #[test]
    fn test_unknkown() {
        let fns = Fns::new();
        let bucket: &Bucket<MessageWithField> = fns.get_slice();

        assert!(bucket.get_helper("unknown-helper").is_none());
        assert!(bucket.get_predicate("unknown-predicate").is_none());
    }
}
