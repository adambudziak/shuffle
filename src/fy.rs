//! Implementation of Fisher-Yates algorithm.
//!

use rand::Rng;

use crate::shuffler::Shuffler;

/// Implementation of Fisher-Yates algorithm.
///
/// # Examples
/// ```
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
    buffer: [u8; std::mem::size_of::<usize>()],
}

impl<T> Shuffler<T> for FisherYates {
    fn shuffle<R>(&mut self, data: &mut Vec<T>, rng: &mut R) -> Result<(), &str>
    where
        T: Clone,
        R: Rng + ?Sized,
    {
        for i in (1..data.len()).rev() {
            let j = rng.gen_range(0..(i + 1));
            data.swap(i, j);
        }
        Ok(())
    }
}
