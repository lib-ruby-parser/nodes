use super::bucket::{GetPartition, PartitionKey};
use super::{bucket::Bucket, F};
use crate::template::GlobalContext;
use crate::{Message, MessageField, Node, NodeField};

#[derive(Default)]
pub struct Fns {
    pub(crate) global: Bucket<GlobalContext>,
    pub(crate) node: Bucket<Node>,
    pub(crate) node_field: Bucket<NodeField>,
    pub(crate) message: Bucket<Message>,
    pub(crate) message_field: Bucket<MessageField>,
}

pub trait BucketKey {
    fn locate(fns: &Fns) -> &Bucket<Self>;
    fn locate_mut(fns: &mut Fns) -> &mut Bucket<Self>;
}

macro_rules! gen_impl {
    ($t:ty, $l:ident) => {
        impl BucketKey for $t {
            fn locate(fns: &Fns) -> &Bucket<Self> {
                &fns.$l
            }

            fn locate_mut(fns: &mut Fns) -> &mut Bucket<Self> {
                &mut fns.$l
            }
        }
    };
}

gen_impl!(GlobalContext, global);
gen_impl!(Node, node);
gen_impl!(NodeField, node_field);
gen_impl!(Message, message);
gen_impl!(MessageField, message_field);

pub trait GetBucket {
    fn get_bucket<T>(&self) -> &Bucket<T>
    where
        T: BucketKey;
    fn get_bucket_mut<T>(&mut self) -> &mut Bucket<T>
    where
        T: BucketKey;
}

impl GetBucket for Fns {
    fn get_bucket<T>(&self) -> &Bucket<T>
    where
        T: BucketKey,
    {
        T::locate(self)
    }

    fn get_bucket_mut<T>(&mut self) -> &mut Bucket<T>
    where
        T: BucketKey,
    {
        T::locate_mut(self)
    }
}

impl Fns {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T, Kind>(&mut self, helper: &str, f: F::HelperFn<T, Kind>)
    where
        T: BucketKey,
        Kind: PartitionKey,
    {
        self.get_bucket_mut::<T>()
            .get_partition_mut::<Kind>()
            .register(helper, f);
    }

    pub fn known<T, Kind>(&self) -> Vec<String>
    where
        T: BucketKey,
        Kind: PartitionKey,
    {
        self.get_bucket::<T>().get_partition::<Kind>().known()
    }

    fn append_to(self, out: &mut Self) {
        self.global.append_to(&mut out.global);
        self.node.append_to(&mut out.node);
        self.node_field.append_to(&mut out.node_field);
        self.message.append_to(&mut out.message);
        self.message_field.append_to(&mut out.message_field);
    }
}

pub trait Dispatch<T, Kind>
where
    T: BucketKey,
    Kind: F::Kind + PartitionKey,
{
    fn dispatch(&self, name: &str, ctx: &T) -> Option<<Kind as F::Kind>::Returns>;
}

fn dispatch0<T, Kind>(fns: &Fns, name: &str, ctx: &T) -> Option<<Kind as F::Kind>::Returns>
where
    Kind: F::Kind + PartitionKey,
    T: BucketKey,
{
    fns.get_bucket::<T>()
        .get_partition::<Kind>()
        .get(name)
        .map(|f| f(ctx))
}

impl<Kind> Dispatch<GlobalContext, Kind> for Fns
where
    Kind: F::Kind + PartitionKey,
{
    fn dispatch(&self, name: &str, ctx: &GlobalContext) -> Option<<Kind as F::Kind>::Returns> {
        dispatch0::<GlobalContext, Kind>(self, name, ctx)
    }
}

impl<Kind> Dispatch<Node, Kind> for Fns
where
    Kind: F::Kind + PartitionKey,
{
    fn dispatch(&self, name: &str, node: &Node) -> Option<<Kind as F::Kind>::Returns> {
        dispatch0::<Node, Kind>(self, name, node)
    }
}

impl<Kind> Dispatch<NodeField, Kind> for Fns
where
    Kind: F::Kind + PartitionKey,
{
    fn dispatch(&self, name: &str, node_field: &NodeField) -> Option<<Kind as F::Kind>::Returns> {
        dispatch0::<NodeField, Kind>(self, name, node_field)
    }
}

impl<Kind> Dispatch<Message, Kind> for Fns
where
    Kind: F::Kind + PartitionKey,
{
    fn dispatch(&self, name: &str, message: &Message) -> Option<<Kind as F::Kind>::Returns> {
        dispatch0::<Message, Kind>(self, name, message)
    }
}

impl<Kind> Dispatch<MessageField, Kind> for Fns
where
    Kind: F::Kind + PartitionKey,
{
    fn dispatch(
        &self,
        name: &str,
        message_field: &MessageField,
    ) -> Option<<Kind as F::Kind>::Returns> {
        dispatch0::<MessageField, Kind>(self, name, message_field)
    }
}

impl std::ops::Add for Fns {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Self::new();
        self.append_to(&mut out);
        rhs.append_to(&mut out);
        out
    }
}

#[cfg(test)]
mod tests {
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
            fields: &[],
            comment: &[],
        };
        fns.register::<Node, F::Helper>("node-helper", node_helper);

        assert_eq!(
            (&fns as &dyn Dispatch<Node, F::Helper>)
                .dispatch("node-helper", &node)
                .unwrap(),
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
            fields: &[],
            comment: &[],
        };
        let message_f = Message {
            camelcase_name: "NotBar",
            fields: &[],
            comment: &[],
        };
        fns.register::<Message, F::Predicate>("message-predicate", message_predicate);

        assert_eq!(
            (&fns as &dyn Dispatch<Message, F::Predicate>)
                .dispatch("message-predicate", &message_t)
                .unwrap(),
            message_predicate(&message_t)
        );
        assert_eq!(
            (&fns as &dyn Dispatch<Message, F::Predicate>)
                .dispatch("message-predicate", &message_f)
                .unwrap(),
            message_predicate(&message_f)
        );
    }

    #[test]
    fn test_unknkown() {
        let fns = Fns::new();
        let message = Message {
            camelcase_name: "Bar",
            fields: &[],
            comment: &[],
        };

        assert!((&fns as &dyn Dispatch<Message, F::Helper>)
            .dispatch("unknown-helper", &message)
            .is_none());
        assert!((&fns as &dyn Dispatch<Message, F::Predicate>)
            .dispatch("unknown-predicate", &message)
            .is_none());
    }

    #[test]
    fn test_add() {
        let mut fns1 = Fns::new();
        let mut fns2 = Fns::new();

        fn test_helper1(message: &Message) -> String {
            format!("test1 {}", message.camelcase_name)
        }

        fn test_helper2(message: &Message) -> String {
            format!("test2 {}", message.camelcase_name)
        }

        fns1.register::<Message, F::Helper>("helper1", test_helper1);
        assert_eq!(
            fns1.known::<Message, F::Helper>(),
            vec![String::from("helper1")]
        );

        fns2.register::<Message, F::Helper>("helper2", test_helper2);
        assert_eq!(
            fns2.known::<Message, F::Helper>(),
            vec![String::from("helper2")]
        );

        let fns = fns1 + fns2;
        assert_eq!(
            {
                let mut known = fns.known::<Message, F::Helper>();
                known.sort();
                known
            },
            vec![String::from("helper1"), String::from("helper2")]
        );
    }
}
