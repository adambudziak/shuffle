//! Crate implementing various kinds of shuffling algorithms
//! such as Inverse Riffle Shuffle (more algorithms coming soon).
//!
//! # Why
//!
//! Currently, the most common way of shuffling a collection is by using
//! [`rand::shuffle`](rand::seq::SliceRandom::shuffle), which is basically
//! [Fisher-Yates](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)
//! algorithm. This is nice, but it requires that you have a good source
//! of random numbers in an arbitrary range.
//!
//! This crate aims to provide good abstractions to shuffle collections when all you have
//! is just a source of randomness. (but we also implement Fisher-Yates, because why not?)
//!
//! Assuming that the source of the randomness is good,
//! all of the shuffling algorithms return a permutation from a uniform distribution.
//! 
//!
//! # Example
//! ```
//! use shuffle::shuffler::Shuffler;
//! use shuffle::irs::Irs;
//! use rand::rngs::mock::StepRng;
//!
//! let mut rng = StepRng::new(2, 13);
//! let mut irs = Irs::default();
//!
//! let mut input = vec![1, 2, 3, 4, 5];
//!
//! irs.shuffle(&mut input, &mut rng);
//! assert_eq!(&input, &[4, 1, 5, 3, 2]);
//! ```

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(intra_doc_link_resolution_failure)]

pub mod shuffler;
pub mod irs;
pub mod fy;
