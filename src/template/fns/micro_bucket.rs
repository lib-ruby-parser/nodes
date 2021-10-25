use super::F;
use std::collections::HashMap;

pub struct Partition<T, Kind>
where
    T: ?Sized,
    Kind: F::Kind,
{
    fns: HashMap<String, F::HelperFn<T, Kind>>,
}

impl<T, Kind> Default for Partition<T, Kind>
where
    Kind: F::Kind,
{
    fn default() -> Self {
        Self {
            fns: HashMap::new(),
        }
    }
}

impl<T, Kind> Partition<T, Kind>
where
    Kind: F::Kind,
{
    pub(crate) fn register(&mut self, name: &str, f: F::HelperFn<T, Kind>) {
        self.fns.insert(name.to_string(), f);
    }

    pub(crate) fn get(&self, name: &str) -> Option<F::HelperFn<T, Kind>> {
        self.fns.get(name).copied()
    }

    pub(crate) fn known(&self) -> Vec<String> {
        self.fns.keys().cloned().collect()
    }

    pub(crate) fn append_to(self, out: &mut Self) {
        for f in self.known() {
            out.register(&f, self.get(&f).unwrap());
        }
    }
}
