use std::{collections::HashSet, ops::Deref, hash::Hash};

pub struct Set<T: Hash + Eq>(HashSet<T>);

impl<T: Hash + Eq> Set<T> {
    pub fn new(s: HashSet<T>) -> Self {
        Self(s)
    }
}

impl<T: Hash + Eq> Deref for Set<T> {
    type Target = HashSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[macro_export]
macro_rules! set {
    [ $($arg:tt)* ] => { {
        let mut s = std::collections::HashSet::new();
        for v in [ $($arg)* ] {
            if ! s.insert(v) {
                eprintln!("duplicate entry in set");
                panic!()
            }
        }
        Set::new(s)
    } }
}

