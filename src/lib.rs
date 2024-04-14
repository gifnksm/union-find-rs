// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! An implementation of the Union-Find datastructure.
//!
//! This crate implements 2 variations of the union find datastructure.
//! `QuickFindUf` implements an always O(1) `find` while `QuickUnionUf` has a
//! `union` operation that is always O(1).
//!
//! There is also the option whether the union operation joins the underlying
//! tree structure by size or by rank via the `UnionBySize`, `UnionByRank`,
//! `UnionByRankSize` and `UnionBySizeRank` structs that need to be passed
//! to the Union Find datastructure.
//!
//! ```
//! use union_find::{UnionFind, UnionBySize, QuickUnionUf};
//!
//! // build a union find datastructure for 10 elements with quick unions,
//! // merge the unions by size.
//! let mut uf = QuickUnionUf::<UnionBySize>::new(10);
//!
//! // initially each element is in it's own set
//! for i in 0..10 {
//!     assert_eq!(uf.find(i), i);
//! }
//!
//! // join sets containing 0 and 1
//! assert!(uf.union(0,1));
//!
//! assert_eq!(uf.find(0), 0);
//! assert_eq!(uf.find(1), 0);
//! for i in 2..10 {
//!     assert_eq!(uf.find(i), i);
//! }
//! ```

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

#[cfg(all(test, feature = "nightly"))]
#[macro_use]
extern crate lazy_static;

mod traits;
pub use crate::traits::{Union, UnionFind, UnionResult};

mod union;
pub use crate::union::{UnionByRank, UnionByRankSize, UnionBySize, UnionBySizeRank};

mod quick_union;
pub use crate::quick_union::QuickUnionUf;
mod quick_find;
pub use crate::quick_find::QuickFindUf;

#[cfg(test)]
mod tests;
