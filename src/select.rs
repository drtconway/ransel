//! Traits for sets supporting the `select` operation.

use crate::{rank::Rank, set::ImpliedSet};

// Operations for sets supporting select.
///
/// The Select trait exists for data structures that support the
/// `select` operation, which for a given value i, returns the i-th smallest
/// value in the implied set (counting from 0).
///
pub trait Select: ImpliedSet {
    /// Return the i-th smallest element of the set (counting from 0).
    ///
    /// `index` is a rank within the implied set: [0, self.count()).
    fn select(&self, index: usize) -> u64;
}

/// We provide a naive implementation of `select_0` with a default implementation
/// in terms of of [`Rank`](crate::rank::Rank).
///
/// While use of a mixture of `rank_1` and `rank_0` is quite common, the use of
/// `select_0` (selecting the i-th smallest element not in the set) is uncommon,
/// so we provide a convenience implementation that uses binary search over ranks.
///
pub trait Select0: ImpliedSet + Rank {
    /// Return the i-th smallest element not in the set (counting from 0).
    /// `index` is a rank-0 within the implied set: [0, self.size() - self.count()).
    fn select_0(&self, index: usize) -> u64 {
        let mut first = 0;
        let mut count = self.size();
        while count > 0 {
            let step = count / 2;
            let x = first + step;
            let r = self.rank_0(x);
            if r <= index {
                first = x + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }
        first - 1
    }
}
