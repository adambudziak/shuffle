//! The `Shuffler` trait.

#[cfg(feature = "rand-0_8")]
use rand_0_8 as rand;

#[cfg(feature = "rand-0_9")]
use rand_0_9 as rand;

/// A trait defining `Shuffler` objects that can be used for shuffling data
/// in various manners
pub trait Shuffler<T> {
    /// Shuffle the passed data in-place using randomness from the provided
    /// `RngCore`.
    fn shuffle<R>(&mut self, data: &mut Vec<T>, rng: &mut R) -> Result<(), &str>
    where
        T: Clone,
        R: rand::RngCore + ?Sized;
}
