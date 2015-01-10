//! Struct and methods for union-find operation.

#![warn(bad_style, missing_docs,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

use std::{mem, usize};
use std::default::Default;
use std::iter::FromIterator;

#[derive(Clone, Show)]
enum UFNode<T> {
    Key(usize),
    Value(T)
}

impl<T> UFNode<T> {
    fn get(&self) -> &T {
        match self {
            &UFNode::Value(ref val) => val,
            &UFNode::Key(_) => panic!()
        }
    }
    fn get_mut(&mut self) -> &mut T {
        match self {
            &mut UFNode::Value(ref mut val) => val,
            &mut UFNode::Key(_) => panic!()
        }
    }
    fn unwrap(self) -> T {
        match self {
            UFNode::Value(val) => val,
            UFNode::Key(_) => panic!()
        }
    }
}

/// The value that can be contained with `UFValue`.
pub trait UFValue: Sized {
    /// Merge two value into one.
    ///
    /// This is used by `UnionFind::union` operation.
    fn merge(lval: Self, rval: Self) -> Merge<Self>;
}

/// Return value of the `UFValue::merege`
#[allow(missing_docs)]
pub enum Merge<T> {
    Left(T), Right(T)
}

/// Reperesents the size of the group.
#[derive(Copy, Clone, Show, Eq, PartialEq, Ord, PartialOrd)]
pub struct Size(usize);

impl UFValue for Size {
    fn merge(Size(lval): Size, Size(rval): Size) -> Merge<Size> {
        if lval >= rval {
            Merge::Left(Size(lval + rval))
        } else {
            Merge::Right(Size(lval + rval))
        }
    }
}

impl Default for Size {
    fn default() -> Size { Size(1) }
}

/// Struct for union-find operation.
#[derive(Clone, Show)]
pub struct UnionFind<V = Size> {
    data: Vec<UFNode<V>>
}

impl<T: UFValue = Size> UnionFind<T> {
    /// Creates empty `UnionFind` struct.
    #[inline]
    pub fn new(len: usize) -> UnionFind<T>
        where T: Default
    {
        UnionFind {
            data: (0 .. len)
                .map(|_| UFNode::Value(Default::default()))
                .collect()
        }
    }

    /// Join two sets that contains given keys (Union operation).
    ///
    /// Returns `true` if these keys are belonged to different sets.
    #[inline]
    pub fn union(&mut self, key0: usize, key1: usize) -> bool {
        let k0 = self.get_key(key0);
        let k1 = self.get_key(key1);
        if k0 == k1 { return false; }

        // Temporary replace with dummy to move out the elements of the vector.
        let v0 = mem::replace(&mut self.data[k0], UFNode::Key(usize::MAX)).unwrap();
        let v1 = mem::replace(&mut self.data[k1], UFNode::Key(usize::MAX)).unwrap();

        match UFValue::merge(v0, v1) {
            Merge::Left(val) => {
                self.data[k0] = UFNode::Value(val);
                self.data[k1] = UFNode::Key(k0);
            }
            Merge::Right(val) => {
                self.data[k1] = UFNode::Value(val);
                self.data[k0] = UFNode::Key(k1);
            }
        }

        true
    }

    /// Returns `true` if two keys contained by the same set (find operation).
    #[inline]
    pub fn find(&mut self, key0: usize, key1: usize) -> bool {
         self.get_key(key0) == self.get_key(key1)
    }

    /// Returns the reference to the value of the set that the key belongs to.
    #[inline]
    pub fn get(&mut self, key: usize) -> &T {
        let root_key = self.get_key(key);
        self.data[root_key].get()
    }

    /// Returns the mutable reference to the value of the set that the key belongs to.
    #[inline]
    pub fn get_mut(&mut self, key: usize) -> &mut T {
        let root_key = self.get_key(key);
        self.data[root_key].get_mut()
    }

    fn get_key(&mut self, key: usize) -> usize {
        let root_key = match self.data[key] {
            UFNode::Value(_) => return key,
            UFNode::Key(key) => self.get_key(key)
        };

        self.data[key] = UFNode::Key(root_key);
        root_key
    }
}

impl<T: UFValue> FromIterator<T> for UnionFind<T> {
    #[inline]
    fn from_iter<I: Iterator<Item = T>>(iterator: I) -> UnionFind<T> {
        UnionFind { data: iterator.map(UFNode::Value).collect() }
    }
}

#[cfg(test)]
mod tests {
    use super::{UnionFind, Size};

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(100);
        assert_eq!(&Size(1), uf.get(0));
        assert_eq!(&Size(1), uf.get(1));
        assert!(!uf.find(0, 1));
        assert!(!uf.find(1, 2));
        assert!(uf.union(0, 1));
        assert!(uf.find(0, 1));
        assert_eq!(&Size(2), uf.get(0));
        assert_eq!(&Size(2), uf.get(1));
        assert_eq!(&Size(1), uf.get(2));
        assert!(!uf.union(0, 1));
        assert_eq!(&Size(2), uf.get(0));
        assert_eq!(&Size(2), uf.get(1));
        assert_eq!(&Size(1), uf.get(2));
        assert!(uf.union(1, 2));
        assert_eq!(&Size(3), uf.get(0));
        assert_eq!(&Size(3), uf.get(1));
        assert_eq!(&Size(3), uf.get(2));
        assert!(uf.find(0, 1));
        assert!(uf.find(2, 1));
    }
}
