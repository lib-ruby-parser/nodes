mod bucket;
#[allow(clippy::module_inception)]
mod fns;
mod kind;
mod micro_bucket;

#[allow(non_snake_case)]
pub mod F {
    pub use super::kind::*;
}
pub use fns::BucketKey;
pub(crate) use fns::Dispatch;
pub use fns::Fns;
