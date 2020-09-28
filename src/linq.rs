use std::slice::Iter;
use std::collections::{HashMap};
use std::hash::Hash;
use std::cmp::Eq;

pub trait GroupBy<'a, T, K: Eq + Hash, F: Fn(&T) -> K> {
    fn group_by(&self, key: F) -> HashMap<K, Vec<T>>;
}

impl<'a, T, K: Eq + Hash, F: Fn(&T) -> K> GroupBy<'a, T, K, F> for Iter<'a, T> {
    fn group_by(&self, key: F) -> HashMap<K, Vec<T>> {
        let ret: HashMap<K, Vec<T>> = HashMap::new();
    
        for elem in *self {
            let k = key(elem);
    
            match ret.get_mut(&k) {
                Some(s) => {
                    s.push(*elem);
                }
                None => {
                    ret.insert(k, vec![*elem]);
                }
            };
        }
    
        ret
    }
}

pub trait Concat<'a, T> {
    fn concat(&self, other: Iter<'a, T>) -> Iter<'a, T>;
}

struct ConcatIter<'a, T> {
    i1: Iter<'a, T>,
    i2: Iter<'a, T>,
}

impl<'a, T> Iterator for ConcatIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.i1.next() {
            Some(s) => Some(*s),
            None => match self.i2.next() {
                Some(s) => Some(*s),
                None => None
            }
        }
    }
}

impl<'a, T> Concat<'a, T> for Iter<'a, T> {
    fn concat(&self, other: Iter<'a, T>) -> Iter<'a, T> {
        ConcatIter {i1: *self, i2: other}
    }
}