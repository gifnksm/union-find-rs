use std::iter::FromIterator;
use std::{mem, usize};
use {UfValue, UnionFind, Merge};

/// Union-Find implementation with quick find operation.
#[derive(Debug)]
pub struct QuickFindUf<V> {
    parent: Vec<usize>,
    data: Vec<(Option<V>, usize)>,
    next: Vec<usize>
}

impl<V> Clone for QuickFindUf<V>
    where V: Clone + UfValue
{
    #[inline]
    fn clone(&self) -> QuickFindUf<V> {
        QuickFindUf {
            parent: self.parent.clone(),
            data: self.data.clone(),
            next: self.next.clone()
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &QuickFindUf<V>) {
        self.parent.clone_from(&other.parent);
        self.data.clone_from(&other.data);
        self.next.clone_from(&other.next);
    }
}

impl<V: UfValue> UnionFind<V> for QuickFindUf<V> {
    #[inline]
    fn size(&self) -> usize { self.data.len() }

    #[inline]
    fn union(&mut self, key0: usize, key1: usize) -> bool {
        let k0 = self.find(key0);
        let k1 = self.find(key1);
        if k0 == k1 { return false; }

        // Temporary replace with dummy to move out the elements of the vector.
        let (v0, l0) = mem::replace(&mut self.data[k0], (None, usize::MAX));
        let (v1, l1) = mem::replace(&mut self.data[k1], (None, usize::MAX));
        let v0 = v0.unwrap();
        let v1 = v1.unwrap();

        let (parent, child, val, last) = match UfValue::merge(v0, v1) {
            Merge::Left(val) => (k0, k1, val, l0),
            Merge::Right(val) => (k1, k0, val, l1)
        };

        self.next[last] = child;

        let mut elem = child;
        while self.next[elem] != elem {
            debug_assert_eq!(self.parent[elem], child);
            self.parent[elem] = parent;
            elem = self.next[elem];
        }
        debug_assert_eq!(self.parent[elem], child);
        self.parent[elem] = parent;
        self.data[parent] = (Some(val), elem);

        true
    }

    #[inline]
    fn find(&mut self, key: usize) -> usize { self.parent[key] }

    #[inline]
    fn get(&mut self, key: usize) -> &V {
        let root_key = self.find(key);
        self.data[root_key].0.as_ref().unwrap()
    }

    #[inline]
    fn get_mut(&mut self, key: usize) -> &mut V {
        let root_key = self.find(key);
        self.data[root_key].0.as_mut().unwrap()
    }
}

impl<A: UfValue> FromIterator<A> for QuickFindUf<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item=A>>(iterator: T) -> QuickFindUf<A> {
        let data = iterator.into_iter().map(Some).zip(0..).collect::<Vec<_>>();
        let len = data.len();
        QuickFindUf {
            parent: (0..len).collect(),
            data: data,
            next: (0..len).collect()
        }
    }
}
