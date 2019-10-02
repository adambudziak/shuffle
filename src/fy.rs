//! Implementation of Fisher-Yates algorithm.
//!

use rand::RngCore;

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
/// assert_eq!(&input, &[3, 4, 2, 5, 1]);
/// ```
#[derive(Debug, Default)]
pub struct FisherYates {
    buffer: [u8; std::mem::size_of::<usize>()],
}

impl<T> Shuffler<T> for FisherYates {
    fn shuffle<R>(&mut self, data: &mut Vec<T>, rng: &mut R) -> Result<(), &str>
    where
        T: Clone,
        R: RngCore + ?Sized
    {
        for i in 1..data.len() {
            let j = self.gen_range(i, rng);
            data.swap(i, j);
        }
        Ok(())
    }
}

impl FisherYates {
    fn gen_range<R>(&mut self, top: usize, rng: &mut R) -> usize
    where
        R: RngCore + ?Sized,
    {
        const USIZE_BYTES : usize = std::mem::size_of::<usize>();
        let bit_width = USIZE_BYTES * 8  - top.leading_zeros() as usize;
        let byte_count = (bit_width - 1) / 8 + 1;
        loop {
            rng.fill_bytes(&mut self.buffer[..byte_count]);
            let result = usize::from_le_bytes(self.buffer);
            let result = result & ((1 << bit_width) - 1);
            if result < top {
                break result
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::rngs::mock::StepRng;
    
    #[test]
    fn test_gen_range() {
        let mut fy = FisherYates::default();
        let mut rng = StepRng::new(13, 7);
        let top = 1;
        assert!(fy.gen_range(top, &mut rng) < top);
    }
}
