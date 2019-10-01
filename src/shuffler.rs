//! The `Shuffler` trait.

use rand::RngCore;

/// A trait defining `Shuffler` objects that can be used for shuffling data
/// in various manners
pub trait Shuffler<T> {
    /// Shuffle the passed data in-place using randomness from the provided
    /// `RngCore`.
    fn shuffle<R>(&mut self, data: &mut Vec<T>, rng: &mut R) -> Result<(), &str>
    where
        T: Clone,
        R: RngCore + ?Sized;
}

