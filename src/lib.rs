//! Struct and methods for union-find operation.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

#![cfg_attr(all(test, feature = "nightly"), feature(test))]

use std::default::Default;
use std::iter::FromIterator;

/// The value that can be contained with `UfValue`.
pub trait UfValue: Sized + Default {
    /// Merge two value into one.
    ///
    /// This is used by `UnionFind::union` operation.
    fn merge(lval: Self, rval: Self) -> Merge<Self>;
}

/// Return value of the `UfValue::merege`
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum Merge<T> {
    Left(T), Right(T)
}

/// Reperesents the size of the group.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Size(usize);

impl UfValue for Size {
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

/// APIs for Union-Find operation.
pub trait UnionFind<V: UfValue>: FromIterator<V> + Sized {
    /// Creates empty `UnionFind` struct.
    #[inline]
    fn new(len: usize) -> Self where V: Default {
        Self::from_iter((0..len).map(|_| Default::default()))
    }

    /// Returns the size of `self`.
    fn size(&self) -> usize;

    /// Join two sets that contains given keys (Union operation).
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

mod quick_union;
pub use quick_union::QuickUnionUf;

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "nightly"))]
mod bench;
