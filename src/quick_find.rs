// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::iter::FromIterator;
use std::mem;
use {Union, UnionFind, UnionResult};

#[derive(Copy, Clone, Debug)]
struct Payload<V> {
    data: V,
    link_last_child: usize,
}

/// Union-Find implementation with quick find operation.
#[derive(Debug)]
pub struct QuickFindUf<V> {
    link_root: Vec<usize>,
    link_sibling: Vec<usize>,
    payload: Vec<Option<Payload<V>>>,
}

impl<V> Clone for QuickFindUf<V>
where
    V: Clone + Union,
{
    #[inline]
    fn clone(&self) -> QuickFindUf<V> {
        QuickFindUf {
            link_root: self.link_root.clone(),
            link_sibling: self.link_sibling.clone(),
            payload: self.payload.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &QuickFindUf<V>) {
        self.link_root.clone_from(&other.link_root);
        self.link_sibling.clone_from(&other.link_sibling);
        self.payload.clone_from(&other.payload);
    }
}

impl<V: Union> UnionFind<V> for QuickFindUf<V> {
    #[inline]
    fn size(&self) -> usize {
        self.payload.len()
    }

    #[inline]
    fn insert(&mut self, data: V) -> usize {
        let key = self.payload.len();
        self.link_root.push(key);
        self.link_sibling.push(key);
        self.payload.push(Some(Payload {
            data: data,
            link_last_child: key,
        }));
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
        let Payload {
            data: d0,
            link_last_child: c0,
        } = mem::replace(&mut self.payload[k0], None).unwrap();
        let Payload {
            data: d1,
            link_last_child: c1,
        } = mem::replace(&mut self.payload[k1], None).unwrap();

        let (root, child_root, val, last) = match Union::union(d0, d1) {
            UnionResult::Left(val) => (k0, k1, val, c0),
            UnionResult::Right(val) => (k1, k0, val, c1),
        };

        self.link_sibling[last] = child_root;

        let mut elem = child_root;
        while self.link_sibling[elem] != elem {
            debug_assert_eq!(self.link_root[elem], child_root);
            self.link_root[elem] = root;
            elem = self.link_sibling[elem];
        }
        debug_assert_eq!(self.link_root[elem], child_root);
        self.link_root[elem] = root;

        self.payload[root] = Some(Payload {
            data: val,
            link_last_child: elem,
        });

        true
    }

    #[inline]
    fn find(&mut self, key: usize) -> usize {
        self.link_root[key]
    }

    #[inline]
    fn get(&mut self, key: usize) -> &V {
        let root_key = self.find(key);
        &self.payload[root_key].as_ref().unwrap().data
    }

    #[inline]
    fn get_mut(&mut self, key: usize) -> &mut V {
        let root_key = self.find(key);
        &mut self.payload[root_key].as_mut().unwrap().data
    }
}

impl<A: Union> FromIterator<A> for QuickFindUf<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = A>>(iterator: T) -> QuickFindUf<A> {
        let mut uf = QuickFindUf {
            link_root: vec![],
            link_sibling: vec![],
            payload: vec![],
        };
        uf.extend(iterator);
        uf
    }
}

impl<A> Extend<A> for QuickFindUf<A> {
    #[inline]
    fn extend<T>(&mut self, iterable: T)
    where
        T: IntoIterator<Item = A>,
    {
        let len = self.payload.len();
        let payload = iterable
            .into_iter()
            .zip(len..)
            .map(|(data, link)| {
                Payload {
                    data: data,
                    link_last_child: link,
                }
            })
            .map(Some);
        self.payload.extend(payload);

        let new_len = self.payload.len();
        self.link_root.extend(len..new_len);
        self.link_sibling.extend(len..new_len);
    }
}
