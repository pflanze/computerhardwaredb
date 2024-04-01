use std::{fmt::Debug, hash::Hash, collections::HashMap};

use anyhow::{bail, Result};

pub trait PrimaryKey<PK> {
    fn primary_key(&self) -> &PK;
}


pub fn pindex_by<K: Debug + Eq + Hash, T: Debug>(
    items: &[T], key: impl Fn(&T) -> &K
) -> Result<HashMap<&K, &T>> {
    let mut m = HashMap::new();
    for item in items {
        if let Some(old) = m.insert(key(item), item) {
            bail!("duplicate unique key {:?} used in item {item:#?} and previously {old:#?}",
                  key(item));
        }
    }
    Ok(m)
}


/// Index by a field that is a foreign key on another index, checking
/// that no foreign keys are broken.
#[allow(unused)]
pub fn pindex_by_foreign<'t, K: Debug + Eq + Hash, T: Debug, T2>(
    items: &'t [T], key: impl Fn(&T) -> &K,
    foreign: &HashMap<&K, T2>, keyname: &str
) -> Result<HashMap<&'t K, &'t T>> {
    let mut m = HashMap::new();
    for item in items {
        let k = key(item);
        if ! foreign.contains_key(&k) {
            bail!("value {k:?} for foreign key {keyname} does not exist");
        }
        if let Some(old) = m.insert(k, item) {
            bail!("duplicate unique key {:?} used in item {item:#?} and previously {old:#?}",
                  key(item));
        }
    }
    Ok(m)
}

/// Index by a field that is a foreign key on another index, checking
/// that no foreign keys are broken.
#[allow(unused)]
pub fn mindex_by_foreign<'t,
                         K: Debug + Eq + Hash,
                         PK: Debug + Eq + Hash,
                         T: Debug + PrimaryKey<PK>,
                         T2>(
    items: &'t [T], key: impl Fn(&T) -> &K,
    foreign: &HashMap<&K, T2>, keyname: &str
) -> Result<HashMap<&'t K, HashMap<&'t PK, &'t T>>> {
    let mut m: HashMap<&'t K, HashMap<&'t PK, &'t T>> = HashMap::new();
    for item in items {
        let k = key(item);
        if ! foreign.contains_key(&k) {
            bail!("value {k:?} for foreign key {keyname} does not exist");
        }
        let pk = item.primary_key();
        if let Some(m2) = m.get_mut(k) {
            if let Some(old) = m2.insert(pk, item) {
                bail!("duplicate primary key {pk:?} used in item {item:#?} and previously {old:#?}");
            }
        } else {
            let mut m2 = HashMap::new();
            m2.insert(pk, item);
            m.insert(k, m2);
        }
    }
    Ok(m)
}
