//! Crate implementing various kinds of shuffling algorithms
//! such as Inverse Riffle Shuffle (more algorithms coming soon).
//!
//! # Why
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
//!
//! # #[cfg(feature = "rand-0_8")]
//! # use rand_0_8 as rand;
//!
//! # #[cfg(feature = "rand-0_9")]
//! # use rand_0_9 as rand;
//!
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

#![no_std]

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod fy;
pub mod irs;
pub mod shuffler;
