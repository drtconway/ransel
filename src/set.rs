
pub trait ImpliedSet {
    /**
     * @return the number of elements in the implied set.
     */
    fn count(&self) -> usize;

    /**
     * @return the size of the domain (1 greater than the largest possible value in the set)
     */
    fn size(&self) -> u64;
}