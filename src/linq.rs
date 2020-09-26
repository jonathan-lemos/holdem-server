use std::slice::Iter;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::cmp::Eq;

pub fn group_by<'a, T: Eq + Hash, TKey: Eq + Hash>(elems: Iter<'a, T>, key: &dyn Fn(&T) -> TKey) -> HashMap<TKey, HashSet<T>> {
    let ret: HashMap<TKey, HashSet<T>> = HashMap::new();

    for elem in elems {
        let k = key(elem);

        match ret.get_mut(&k) {
            Some(s) => {
                s.insert(*elem);
            }
            None => {
                let hn = HashSet::new();
                hn.insert(*elem);
                ret.insert(*key, hn);
                ()
            }
        }
    }

    ret
}