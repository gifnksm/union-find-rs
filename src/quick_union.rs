// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::iter::FromIterator;
use std::mem;
use {Union, UnionFind, UnionResult};

/// Union-Find implementation with quick union operation.
#[derive(Debug)]
pub struct QuickUnionUf<V> {
    link_parent: Vec<usize>,
    payload: Vec<Option<V>>,
}

impl<V: Clone> Clone for QuickUnionUf<V> {
    #[inline]
    fn clone(&self) -> QuickUnionUf<V> {
        QuickUnionUf {
            link_parent: self.link_parent.clone(),
            payload: self.payload.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &QuickUnionUf<V>) {
        self.link_parent.clone_from(&other.link_parent);
        self.payload.clone_from(&other.payload);
    }
}

impl<V: Union> UnionFind<V> for QuickUnionUf<V> {
    #[inline]
    fn size(&self) -> usize {
        self.payload.len()
    }

    #[inline]
    fn insert(&mut self, data: V) -> usize {
        let key = self.payload.len();
        self.link_parent.push(key);
        self.payload.push(Some(data));
        key
    }

    #[inline]
    fn union(&mut self, key0: usize, key1: usize) -> bool {
        let k0 = self.find(key0);
        let k1 = self.find(key1);
        if k0 == k1 {
            return false;
        }

        // Temporary replace with dummy to move out the elements of the vector.
        let v0 = mem::replace(&mut self.payload[k0], None).unwrap();
        let v1 = mem::replace(&mut self.payload[k1], None).unwrap();

        let (parent, child, val) = match Union::union(v0, v1) {
            UnionResult::Left(val) => (k0, k1, val),
            UnionResult::Right(val) => (k1, k0, val),
        };
        self.payload[parent] = Some(val);
        self.link_parent[child] = parent;

        true
    }

    #[inline]
    fn find(&mut self, key: usize) -> usize {
        let mut k = key;
        let mut p = self.link_parent[k];
        while p != k {
            let pp = self.link_parent[p];
            self.link_parent[k] = pp;
            k = p;
            p = pp;
        }
        k
    }

    #[inline]
    fn get(&mut self, key: usize) -> &V {
        let root_key = self.find(key);
        self.payload[root_key].as_ref().unwrap()
    }

    #[inline]
    fn get_mut(&mut self, key: usize) -> &mut V {
        let root_key = self.find(key);
        self.payload[root_key].as_mut().unwrap()
    }
}

impl<A: Union> FromIterator<A> for QuickUnionUf<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = A>>(iterator: T) -> QuickUnionUf<A> {
        let mut uf = QuickUnionUf {
            link_parent: vec![],
            payload: vec![],
        };
        uf.extend(iterator);
        uf
    }
}

impl<A> Extend<A> for QuickUnionUf<A> {
    #[inline]
    fn extend<T>(&mut self, iterable: T)
    where
        T: IntoIterator<Item = A>,
    {
        let len = self.payload.len();
        let payload = iterable.into_iter().map(Some);
        self.payload.extend(payload);

        let new_len = self.payload.len();
        self.link_parent.extend(len..new_len);
    }
}
