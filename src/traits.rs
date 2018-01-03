// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::iter::FromIterator;

/// The value that can be contained with `Union`.
pub trait Union: Sized {
    /// Union two value into one.
    ///
    /// This is used by `UnionFind::union` operation.
    fn union(lval: Self, rval: Self) -> UnionResult<Self>;
}

/// Return value of the `Union::merge`.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum UnionResult<T> {
    Left(T),
    Right(T),
}

/// APIs for Union-Find operation.
pub trait UnionFind<V: Union>: FromIterator<V> + Extend<V> + Sized {
    /// Creates empty `UnionFind` struct.
    #[inline]
    fn new(len: usize) -> Self
    where
        V: Default,
    {
        Self::from_iter((0..len).map(|_| Default::default()))
    }

    /// Returns the size of `self`.
    fn size(&self) -> usize;

    /// Inserts a new set into the union.
    ///
    /// Returns the key of the inserted set.
    fn insert(&mut self, data: V) -> usize;

    /// Join two sets that contains given keys (union operation).
    ///
    /// Returns `true` if these keys are belonged to different sets.
    fn union(&mut self, key0: usize, key1: usize) -> bool;

    /// Returns the identifier of the set that the key belongs to.
    fn find(&mut self, key: usize) -> usize;

    /// Returns the reference to the value of the set that the key belongs to.
    fn get(&mut self, key: usize) -> &V;

    /// Returns the mutable reference to the value of the set that the key belongs to.
    fn get_mut(&mut self, key: usize) -> &mut V;
}
