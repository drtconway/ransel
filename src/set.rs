//! Traits for implicit set operations.

/// The trait `ImpliedSet` exposes a basic set-like property of a data structure
/// over non-negative integers.
/// 
pub trait ImpliedSet {
    /// Return the number of elements in the set.
    fn count(&self) -> usize;

    /// Return the size of the domain of the set.
    ///
    /// Must be at least 1 greater than the largest element in the set.
    fn size(&self) -> u64;
}