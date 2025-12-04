//! Implementation of Fisher-Yates algorithm.
//!

extern crate alloc;

use alloc::vec::Vec;

#[cfg(feature = "rand-0_8")]
use rand_0_8 as rand;

#[cfg(feature = "rand-0_9")]
use rand_0_9 as rand;

use crate::shuffler::Shuffler;

/// Implementation of Fisher-Yates algorithm.
///
/// # Examples
/// ```
/// # #[cfg(feature = "rand-0_8")]
/// # use rand_0_8 as rand;
///
/// # #[cfg(feature = "rand-0_9")]
/// # use rand_0_9 as rand;
/// use shuffle::shuffler::Shuffler;
/// use shuffle::fy::FisherYates;
/// use rand::rngs::mock::StepRng;
///
/// let mut rng = StepRng::new(2, 13);
/// let mut fy = FisherYates::default();
///
/// let mut input = vec![1, 2, 3, 4, 5];
///
/// fy.shuffle(&mut input, &mut rng);
/// assert_eq!(&input, &[2, 3, 4, 5, 1]);
/// ```
#[derive(Debug, Default)]
pub struct FisherYates {
}

impl<T> Shuffler<T> for FisherYates {
    fn shuffle<R>(&mut self, data: &mut Vec<T>, rng: &mut R) -> Result<(), &str>
    where
        T: Clone,
        R: rand::Rng + ?Sized,
    {
        for i in (1..data.len()).rev() {
            #[cfg(feature = "rand-0_8")]
            let j = rng.gen_range(0..(i + 1));

            #[cfg(feature = "rand-0_9")]
            let j = rng.random_range(0..(i + 1));
            data.swap(i, j);
        }
        Ok(())
    }
}
