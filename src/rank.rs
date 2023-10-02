//! Traits for sets supporting the `rank` operation.

use crate::set::ImpliedSet;


/// Operations for sets supporting rank.
///
/// The Rank trait exists for data structures that support the
/// rank operation, which for a given value, returns the number
/// of elements in the implied set with a value less than the
/// given one.
///
/// There are a few auxiliary functions which have default implementations
/// in terms of rank, but which may have more efficient implementations
/// for a given underlying representation.
///
/// Note that the domain is over u64 and the range over usize.
/// 
pub trait Rank: ImpliedSet {

    /// Rank returns the number of elements of the implied set strictly
    /// less than the given value.
    ///
    /// `value` is an element of the domain [0, self.size()).
    fn rank(&self, value: u64) -> usize;

    /// `rank_1` is an alias for `rank`, which may be useful to distinguish it
    /// from `rank_0` in contexts where both are used.
    #[inline]
    fn rank_1(&self, value: u64) -> usize {
        self.rank(value)
    }

    /// Return the number of elements less than `value` that are *not* in the set.
    /// 
    /// `s.rank_0(x) == x - s.rank_1(x)` and vice versa.
    #[inline]
    fn rank_0(&self, value: u64) -> usize {
        (value as usize) - self.rank_1(value)
    }

    /// Compute the ranks of two elements of the domain. Implementations may
    /// assume that the two elements are close in value, or close in rank.
    /// 
    /// It is a requirement that `value_1` is strictly less than `value_2`.
    /// 
    #[inline]
    fn rank_2(&self, value_1: u64, value_2: u64) -> (usize, usize) {
        std::debug_assert!(value_1 < value_2);
        (self.rank(value_1), self.rank(value_2))
    }

    /// Return true if `value` is in the implied set.
    fn contains(&self, value: u64) -> bool {
        let (r1, r2) = self.rank_2(value, value + 1);
        r1 < r2
    }

    /// Return the rank of `value` and whether it is contained in the implied set.
    fn access_and_rank(&self, value: u64) -> (usize, bool) {
        let (rank_1, rank_2) = self.rank_2(value, value + 1);
        (rank_1, rank_1 < rank_2)
    }
}