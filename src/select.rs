use crate::{set::ImpliedSet, rank::Rank};


/// * The Select trait exists for data structures that support the
///  * select operation, which for a given value i, returns the ith smallest
///  * value in the implied set.
///  * 
///  * Note that the domain is over u64 and the range over usize. In practice
///  * we expect these will usually both be 64-bit unsigned quantities, so
///  * casts between them will be free, but we use them to avoid confusion
///  * between variables over the domain and range.
pub trait Select: ImpliedSet {

    /// * @param index is a rank within the implied set: [0, self.count()).
    fn select(&self, index: usize) -> u64;
}

pub trait Select0: ImpliedSet + Rank {
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