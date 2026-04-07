// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::{Union, UnionResult};
use std::cmp::Ordering;

const DEFAULT_RANK: u8 = 0;
const DEFAULT_SIZE: usize = 1;

/// Operates the `union` with using the size of the sets as weight.
///
/// A smaller sized set will be the children of a larger sized set.
#[derive(Copy, Clone, Debug)]
pub struct UnionBySize {
    size: usize,
}

impl Union for UnionBySize {
    #[inline]
    fn union(left: UnionBySize, right: UnionBySize) -> UnionResult<UnionBySize> {
        let lsize = left.size();
        let rsize = right.size();
        let result = UnionBySize {
            size: lsize + rsize,
        };
        if lsize >= rsize {
            UnionResult::Left(result)
        } else {
            UnionResult::Right(result)
        }
    }
}

impl Default for UnionBySize {
    #[inline]
    fn default() -> UnionBySize {
        UnionBySize { size: DEFAULT_SIZE }
    }
}

impl UnionBySize {
    /// Returns the size of the set.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Operates the `union` with using the rank of the sets as weight.
///
/// A smaller ranked set will be the children of a larger ranked set.
/// If both sets have the same rank, the rank of the resulting set is incremented.
#[derive(Copy, Clone, Debug)]
pub struct UnionByRank {
    rank: u8,
}

impl Union for UnionByRank {
    #[inline]
    fn union(left: UnionByRank, right: UnionByRank) -> UnionResult<UnionByRank> {
        let lrank = left.rank();
        let rrank = right.rank();
        match lrank.cmp(&rrank) {
            Ordering::Less => UnionResult::Right(right),
            Ordering::Greater => UnionResult::Left(left),
            Ordering::Equal => UnionResult::Left(UnionByRank { rank: lrank + 1 }),
        }
    }
}

impl Default for UnionByRank {
    fn default() -> Self {
        Self { rank: DEFAULT_RANK }
    }
}

impl UnionByRank {
    /// Returns the rank of the set.
    #[inline]
    pub fn rank(&self) -> u8 {
        self.rank
    }
}

/// Operates the `union` with using the size and the rank of the sets as weight.
///
/// A smaller sized set will be the children of a larger sized set.
/// If both sets have the same size, compared by the rank.
#[derive(Copy, Clone, Debug)]
pub struct UnionBySizeRank {
    size: usize,
    rank: u8,
}

impl Union for UnionBySizeRank {
    #[inline]
    fn union(left: UnionBySizeRank, right: UnionBySizeRank) -> UnionResult<UnionBySizeRank> {
        let lsize = left.size();
        let lrank = left.rank();
        let rsize = right.size();
        let rrank = right.rank();

        let rank_cmp = lrank.cmp(&rrank);
        let new_size = lsize + rsize;
        let new_rank = match rank_cmp {
            Ordering::Less => rrank,
            Ordering::Greater => lrank,
            Ordering::Equal => lrank + 1,
        };

        let result = UnionBySizeRank {
            size: new_size,
            rank: new_rank,
        };
        match lsize.cmp(&rsize) {
            Ordering::Less => UnionResult::Right(result),
            Ordering::Greater => UnionResult::Left(result),
            Ordering::Equal => match rank_cmp {
                Ordering::Less => UnionResult::Right(result),
                _ => UnionResult::Left(result),
            },
        }
    }
}

impl Default for UnionBySizeRank {
    #[inline]
    fn default() -> UnionBySizeRank {
        UnionBySizeRank {
            size: DEFAULT_SIZE,
            rank: DEFAULT_RANK,
        }
    }
}

impl UnionBySizeRank {
    /// Returns the size of the set.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the rank of the set.
    #[inline]
    pub fn rank(&self) -> u8 {
        self.rank
    }
}

/// Operates the `union` with using the rank and the size of the sets as weight.
///
/// A smaller ranked set will be the children of a larger ranked set.
/// If both sets have the same rank, compared by the size.
#[derive(Copy, Clone, Debug)]
pub struct UnionByRankSize {
    rank: u8,
    size: usize,
}

impl Union for UnionByRankSize {
    #[inline]
    fn union(left: UnionByRankSize, right: UnionByRankSize) -> UnionResult<UnionByRankSize> {
        let lrank = left.rank();
        let lsize = left.size();
        let rrank = right.rank();
        let rsize = right.size();

        let rank_cmp = lrank.cmp(&rrank);
        let new_size = lsize + rsize;
        let new_rank = match rank_cmp {
            Ordering::Less => rrank,
            Ordering::Greater => lrank,
            Ordering::Equal => lrank + 1,
        };

        let result = UnionByRankSize {
            rank: new_rank,
            size: new_size,
        };
        match rank_cmp {
            Ordering::Less => UnionResult::Right(result),
            Ordering::Greater => UnionResult::Left(result),
            Ordering::Equal => match lsize.cmp(&rsize) {
                Ordering::Less => UnionResult::Right(result),
                _ => UnionResult::Left(result),
            },
        }
    }
}

impl Default for UnionByRankSize {
    #[inline]
    fn default() -> UnionByRankSize {
        UnionByRankSize {
            rank: DEFAULT_RANK,
            size: DEFAULT_SIZE,
        }
    }
}

impl UnionByRankSize {
    /// Returns the rank of the set.
    #[inline]
    pub fn rank(&self) -> u8 {
        self.rank
    }

    /// Returns the size of the set.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
}
