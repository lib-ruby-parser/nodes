pub struct Helper;
pub struct Predicate;

pub trait Kind {
    type Returns;
}

impl Kind for Helper {
    type Returns = String;
}

impl Kind for Predicate {
    type Returns = bool;
}

pub type HelperFn<T, K> = fn(&T) -> <K as Kind>::Returns;
