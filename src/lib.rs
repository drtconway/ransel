#![warn(missing_docs)]

//! The `ransel` library provides a rank/select API to sets of integers originally
//! due to Guy Jacobson:
//! 
//! > Jacobson, G., 1989, October. Space-efficient static trees and graphs.
//! > In 30th annual symposium on foundations of computer science (pp. 549-554).
//! > IEEE Computer Society.
//! 
//! The beauty of the rank/select API is that it allows many useful operations to be
//! performed over sets of integers (or bitmaps, depending on your point of view),
//! without needing to think about the underlying representation of the set. Indeed,
//! there are many different representations that can be used depending on the expected
//! density and size of the set.
//! 
//! Various forms of this API have been presented (notably Simon Gog's 
//! [sdsl-lite](https://github.com/simongog/sdsl-lite)), with slightly varying
//! definitions. The definition we use is uniformly 0-based.
//! 
//! Given a set `S` of non-negative integers over a finite domain, we define:
//! 
//! `S.size()` is the size of the domain, and is at least 1 greater than the largest
//! element in `S`.
//! 
//! `S.count()` is the number of elements in `S`.
//! 
//! `S.rank(x)` is the number of elements in `S` that are strictly less than `x`.
//! `rank_1` is an alias, and `rank_0` is the complementary definition: the number
//! of non-negative integers less than `x` that are *not* in `S` (n.b. `rank_0` and
//! `rank_1` may be defined in terms of each other, with `S.rank_0(x) = x - S.rank_1(x)).
//! 
//! `S.select(i)` is the i-th smallest element in `S`, with `0` being the index of the
//! first element.
//! 
//! The API is broken down in to components:
//! 
//! The [`ImpliedSet`](crate::set::ImpliedSet) trait exposes the `size` and `count` operations.
//! 
//! The [`Rank`](crate::rank::Rank) trait exposes `rank` and its associated operations.
//! 
//! The [`Select`](crate::select::Select) trait exposes `select` and its associated operations.
//! 

pub mod set;
pub mod rank;
pub mod select;
pub mod sparse;
pub mod naive_dense;
pub mod naive_sparse;
pub mod sorted;
pub mod intvec;
pub mod bitvec;
mod persist;
mod words;
mod dense64;
