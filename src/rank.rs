use crate::set::ImpliedSet;


/// The Rank trait exists for data structures that support the
/// rank operation, which for a given value, returns the number
/// of elements in the implied set with a value less than the
/// given one.
///
/// There are a few auxiliary functions which have default implementations
/// in terms of rank, but which may have more efficient implementations
/// for a given underlying representation.
///
/// Note that the domain is over u64 and the range over usize. In practice
/// we expect these will usually both be 64-bit unsigned quantities, so
/// casts between them will be free, but we use them to avoid confusion
/// between variables over the domain and range.
pub trait Rank: ImpliedSet {

    /// Rank returns the number of elements of the implied set strictly
    /// less than the given value.
    ///
    /// @param value is an element of the domain [0, self.size()).
    /// @return the number of elements of the implied set less than the given value.
    fn rank(&self, value: u64) -> usize;

    // This is just an alias for rank, for clarity when mixing rank_1 and rank_0.
    fn rank_1(&self, value: u64) -> usize {
        self.rank(value)
    }

    /// Return the rank of `value` in the complement of the implied set.
    fn rank_0(&self, value: u64) -> usize {
        (value as usize) - self.rank(value)
    }

    /// Compute the ranks of two elements of the domain. Implementations may
    /// assume that the two elements are close in value, or close in rank.
    /// 
    /// It is a requirement that `value_1` is strictly less than `value_2`.
    /// 
    /// @param value_1 is the first element.
    /// @param value_2 is the second element.
    /// @return the pair of ranks.
    fn rank_2(&self, value_1: u64, value_2: u64) -> (usize, usize) {
        std::debug_assert!(value_1 < value_2);
        (self.rank(value_1), self.rank(value_2))
    }

    /// Return true if `value` is in the implied set.
    fn contains(&self, value: u64) -> bool {
        self.rank(value) < self.rank(value + 1)
    }

    /// Return the rank of `value` and whether it is contained in the implied set.
    fn access_and_rank(&self, value: u64) -> (usize, bool) {
        let (rank_1, rank_2) = self.rank_2(value, value + 1);
        (rank_1, rank_1 < rank_2)
    }
}