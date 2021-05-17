// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

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

#[cfg(all(test, feature = "nightly"))]
#[macro_use]
extern crate lazy_static;

mod traits;
pub use traits::{Union, UnionFind, UnionResult};

mod union;
pub use union::{UnionByRank, UnionByRankSize, UnionBySize, UnionBySizeRank};

mod quick_union;
pub use quick_union::QuickUnionUf;
mod quick_find;
pub use quick_find::QuickFindUf;

#[cfg(test)]
mod tests;
#[cfg(all(test, feature = "nightly"))]
mod bench;
