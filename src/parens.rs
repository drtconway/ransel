use crate::rank::Rank;


pub trait BalencedPArens: Rank {

    /// Scan the bits to check they are properly balanced.
    fn valid(&self) -> bool {
        if 2 * self.count() as u64 != self.size() {
            return false;
        }
        for i in 0..self.size() {
            let (r, p) = self.access_and_rank(i);
            if (2*r as u64) < i {
                return false;
            }
        }
        true
    }

    fn parent(&self, value: u64) -> Option<u64> {
        self.enclose(value)
    }

    fn first_child(&self, value: u64) -> Option<u64> {
        Some(value + 1)
    }

    fn close(&self, i: u64) -> Option<u64> {
        self.fwd_search(i, -1)
    }

    fn open(&self, i: u64) -> Option<u64> {
        self.bwd_search(i, 0).map(|j| j + 1)
    }

    fn enclose(&self, i: u64) -> Option<u64> {
        self.bwd_search(i, -2).map(|j| j + 1)
    }

    fn excess(&self, value: u64) -> u64 {
        2 * self.rank(value) as u64 - value
    }

    fn fwd_search(&self, i: u64, d: i64) -> Option<u64> {
        let excess_i = self.excess(i);
        let mut j = i + 1;
        while j < self.size() {
            let excess_j = self.excess(j);
            if  excess_j == (excess_i as i64 + d) as u64 {
                return Some(j)
            }
            j += 1;
        }
        None
    }

    fn bwd_search(&self, i: u64, d: i64) -> Option<u64> {
        let excess_i = self.excess(i);
        let mut j = i;
        while j > 0 {
            j -= 1;
            let excess_j = self.excess(j);
            if  excess_j == (excess_i as i64 + d) as u64 {
                return Some(j)
            }
        }
        None
    }
}