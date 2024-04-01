use std::cmp::Ordering;

use anyhow::Result;


pub trait CollectSorted<T> {
    fn try_collect_sorted_by(&mut self, cmp: impl Fn(&T, &T) -> Ordering) -> Result<Vec<T>>;
}


impl<T, I: Iterator<Item = Result<T>>> CollectSorted<T> for I {
    fn try_collect_sorted_by(&mut self, cmp: impl Fn(&T, &T) -> Ordering) -> Result<Vec<T>> {
        let mut vs = self.collect::<Result<Vec<T>>>()?;
        vs.sort_by(cmp);
        Ok(vs)
    }
}


pub fn on<T, K>(
    key: impl Fn(&T) -> &K,
    cmp: impl Fn(&K, &K) -> Ordering
) -> impl Fn(&T, &T) -> Ordering {
    move |a, b| {
        cmp(key(a), key(b))
    }
}


/// A comparison on PartialOrd that panics when it can't compare values
pub fn unsafe_cmp<T: PartialOrd>(a: &T, b: &T) -> Ordering {
    a.partial_cmp(b).expect("unsafe_cmp expects all instances of the arguments to be comparable")
}

