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
    node_with_field: Bucket<NodeWithField>,
    message: Bucket<Message>,
    message_with_field: Bucket<MessageWithField>,
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
    fn dispatch_helper(&self, fns: &Fns, helper: &str) -> Option<String>;
    fn dispatch_predicate(&self, fns: &Fns, predicate: &str) -> Option<bool>;
}

impl FnSubject for GlobalContext {
    fn get(fns: &Fns) -> &Bucket<Self> {
        &fns.global
    }

    fn get_mut(fns: &mut Fns) -> &mut Bucket<Self> {
        &mut fns.global
    }

    fn dispatch_helper(&self, fns: &Fns, helper: &str) -> Option<String> {
        let helper = Self::get(fns).get_helper(helper)?;
        Some(helper(self))
    }

    fn dispatch_predicate(&self, fns: &Fns, predicate: &str) -> Option<bool> {
        let predicate = Self::get(fns).get_predicate(predicate)?;
        Some(predicate(self))
    }
}
impl FnSubject for Node {
    fn get(fns: &Fns) -> &Bucket<Self> {
        &fns.node
    }

    fn get_mut(fns: &mut Fns) -> &mut Bucket<Self> {
        &mut fns.node
    }

    fn dispatch_helper(&self, fns: &Fns, helper: &str) -> Option<String> {
        Self::get(fns).get_helper(helper).map(|f| f(self))
    }

    fn dispatch_predicate(&self, fns: &Fns, predicate: &str) -> Option<bool> {
        Self::get(fns).get_predicate(predicate).map(|f| f(self))
    }
}
impl FnSubject for NodeWithField {
    fn get(fns: &Fns) -> &Bucket<Self> {
        &fns.node_with_field
    }

    fn get_mut(fns: &mut Fns) -> &mut Bucket<Self> {
        &mut fns.node_with_field
    }

    fn dispatch_helper(&self, fns: &Fns, helper: &str) -> Option<String> {
        // get NodeWithField own helper
        Self::get(fns)
            .get_helper(helper)
            .map(|f| f(self))
            .or_else(|| {
                // or get Node helper
                Node::get(fns).get_helper(helper).map(|f| f(&self.node))
            })
    }

    fn dispatch_predicate(&self, fns: &Fns, predicate: &str) -> Option<bool> {
        // get NodeWithField own predicate
        Self::get(fns)
            .get_predicate(predicate)
            .map(|f| f(self))
            .or_else(|| {
                // or get Node predicate
                Node::get(fns)
                    .get_predicate(predicate)
                    .map(|f| f(&self.node))
            })
    }
}
impl FnSubject for Message {
    fn get(fns: &Fns) -> &Bucket<Self> {
        &fns.message
    }

    fn get_mut(fns: &mut Fns) -> &mut Bucket<Self> {
        &mut fns.message
    }

    fn dispatch_helper(&self, fns: &Fns, helper: &str) -> Option<String> {
        Self::get(fns).get_helper(helper).map(|f| f(self))
    }

    fn dispatch_predicate(&self, fns: &Fns, predicate: &str) -> Option<bool> {
        Self::get(fns).get_predicate(predicate).map(|f| f(self))
    }
}
impl FnSubject for MessageWithField {
    fn get(fns: &Fns) -> &Bucket<Self> {
        &fns.message_with_field
    }

    fn get_mut(fns: &mut Fns) -> &mut Bucket<Self> {
        &mut fns.message_with_field
    }

    fn dispatch_helper(&self, fns: &Fns, helper: &str) -> Option<String> {
        // get MessageWithField own helper
        Self::get(fns)
            .get_helper(helper)
            .map(|f| f(self))
            .or_else(|| {
                // or get Message helper
                Message::get(fns)
                    .get_helper(helper)
                    .map(|f| f(&self.message))
            })
    }

    fn dispatch_predicate(&self, fns: &Fns, predicate: &str) -> Option<bool> {
        // get MessageWithField own predicate
        Self::get(fns)
            .get_predicate(predicate)
            .map(|f| f(self))
            .or_else(|| {
                // or get Message predicate
                Message::get(fns)
                    .get_predicate(predicate)
                    .map(|f| f(&self.message))
            })
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

        assert_eq!(
            node.dispatch_helper(&fns, "node-helper").unwrap(),
            node_helper(&node)
        );
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

        assert_eq!(
            message_t
                .dispatch_predicate(&fns, "message-predicate")
                .unwrap(),
            message_predicate(&message_t)
        );
        assert_eq!(
            message_f
                .dispatch_predicate(&fns, "message-predicate")
                .unwrap(),
            message_predicate(&message_f)
        );
    }

    #[test]
    fn test_unknkown() {
        let fns = Fns::new();
        let message = Message {
            camelcase_name: "Bar",
            fields: MessageFieldList(&[]),
            comment: &[],
        };

        assert!(message.dispatch_helper(&fns, "unknown-helper").is_none());
        assert!(message
            .dispatch_predicate(&fns, "unknown-predicate")
            .is_none());
    }
}
