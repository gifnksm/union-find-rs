//! Struct and methods for union-find operation.

#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

#![cfg_attr(all(test, feature = "nightly"), feature(test))]
#![cfg_attr(all(test, feature = "nightly"), feature(const_fn))]

#[cfg(all(test, feature = "nightly"))]
#[macro_use]
#[no_link]
extern crate lazy_static;

mod traits;
pub use traits::{Union, UnionFind, UnionResult};

mod union;
pub use union::{UnionBySize, UnionByRank, UnionBySizeRank, UnionByRankSize};

mod quick_union;
pub use quick_union::QuickUnionUf;
mod quick_find;
pub use quick_find::QuickFindUf;

#[cfg(test)]
mod tests;
#[cfg(all(test, feature = "nightly"))]
mod bench;
