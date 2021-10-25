use super::{micro_bucket::Partition, F};

pub struct Bucket<T>
where
    T: ?Sized,
{
    pub(crate) helpers: Partition<T, F::Helper>,
    pub(crate) predicates: Partition<T, F::Predicate>,
}

impl<T> Default for Bucket<T> {
    fn default() -> Self {
        Self {
            helpers: Partition::default(),
            predicates: Partition::default(),
        }
    }
}

impl<T> Bucket<T> {
    pub(crate) fn append_to(self, out: &mut Self) {
        self.helpers.append_to(&mut out.helpers);
        self.predicates.append_to(&mut out.predicates);
    }
}

pub trait PartitionKey: F::Kind {
    fn locate<T>(bucket: &Bucket<T>) -> &Partition<T, Self>
    where
        T: ?Sized,
        Self: Sized;
    fn locate_mut<T>(bucket: &mut Bucket<T>) -> &mut Partition<T, Self>
    where
        T: ?Sized,
        Self: Sized;
}

impl PartitionKey for F::Helper {
    fn locate<T>(bucket: &Bucket<T>) -> &Partition<T, Self>
    where
        T: ?Sized,
    {
        &bucket.helpers
    }

    fn locate_mut<T>(bucket: &mut Bucket<T>) -> &mut Partition<T, Self>
    where
        T: ?Sized,
    {
        &mut bucket.helpers
    }
}

impl PartitionKey for F::Predicate {
    fn locate<T>(bucket: &Bucket<T>) -> &Partition<T, Self>
    where
        T: ?Sized,
    {
        &bucket.predicates
    }

    fn locate_mut<T>(bucket: &mut Bucket<T>) -> &mut Partition<T, Self>
    where
        T: ?Sized,
    {
        &mut bucket.predicates
    }
}

pub trait GetPartition<T>
where
    T: ?Sized,
{
    fn get_partition<Kind>(&self) -> &Partition<T, Kind>
    where
        Kind: PartitionKey;
    fn get_partition_mut<Kind>(&mut self) -> &mut Partition<T, Kind>
    where
        Kind: PartitionKey;
}

impl<T> GetPartition<T> for Bucket<T>
where
    T: ?Sized,
{
    fn get_partition<Kind>(&self) -> &Partition<T, Kind>
    where
        Kind: PartitionKey,
    {
        Kind::locate(self)
    }

    fn get_partition_mut<Kind>(&mut self) -> &mut Partition<T, Kind>
    where
        Kind: PartitionKey,
    {
        Kind::locate_mut(self)
    }
}
