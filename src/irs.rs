//! Implementation of the Inverse Riffle Shuffle algorithm.
//!
//! A Riffle Shuffle is a common algorithm used for mixing cards
//! (see [wiki](https://en.wikipedia.org/wiki/Riffle_shuffle_permutation)).
//!
//! Inverse Riffle Shuffle is an algorithm that has exactly the same
//! properties as Riffle Shuffle, but is much more implementation-friendly.
//!
//! *Aldous, David, and Persi Diaconis. "Shuffling cards and stopping times."
//! The American Mathematical Monthly 93.5 (1986): 333-348.*

extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;

use bitvec::order::Lsb0;
use bitvec::prelude::*;

#[cfg(feature = "rand-0_8")]
use rand_0_8 as rand;

#[cfg(feature = "rand-0_9")]
use rand_0_9 as rand;

use crate::shuffler::Shuffler;

#[derive(Eq, PartialEq, Debug)]
struct Context<T> {
    data_cp: Vec<T>,
    bit_slots: Vec<u64>,
    bit_slots_cp: Vec<u64>,
}

impl<T> Context<T>
where
    T: Clone,
{
    fn new(data: &[T]) -> Context<T> {
        Context {
            data_cp: data.to_vec(),
            bit_slots: vec![0; data.len()],
            bit_slots_cp: vec![0; data.len()],
        }
    }
}

struct InfiniteBitIter<'a, R>
where
    R: ?Sized,
{
    buffer: BitVec<Lsb0, u8>,
    rng: &'a mut R,
    index: usize,
}

impl<'a, R> InfiniteBitIter<'a, R>
where
    R: ?Sized,
{
    fn new(buffer: Vec<u8>, rng: &'a mut R) -> Self {
        Self {
            buffer: BitVec::from_vec(buffer),
            rng,
            index: 0,
        }
    }
}

impl<'a, R> InfiniteBitIter<'a, R>
where
    R: rand::RngCore + ?Sized,
{
    fn next_bit(&mut self) -> bool {
        let cbuf_bits = self.buffer.len() * 8;
        if self.index == cbuf_bits {
            self.index = 0;
            self.rng.fill_bytes(self.buffer.as_mut_slice());
        }
        // This is safe because we manually check whether the index
        // is still in the range.
        let result = unsafe { self.buffer.get_unchecked(self.index) };
        self.index += 1;
        *result
    }
}

/// Implementation of Inverse Riffle Shuffle.
///
/// # Examples
/// ```
/// # #[cfg(feature = "rand-0_8")]
/// # use rand_0_8 as rand;
///
/// # #[cfg(feature = "rand-0_9")]
/// # use rand_0_9 as rand;
///
/// use shuffle::shuffler::Shuffler;
/// use shuffle::irs::Irs;
/// use rand::rngs::mock::StepRng;
///
/// let mut rng = StepRng::new(2, 13);
/// let mut irs = Irs::default();
///
/// let mut input = vec![1, 2, 3, 4, 5];
///
/// irs.shuffle(&mut input, &mut rng);
/// assert_eq!(&input, &[4, 1, 5, 3, 2]);
/// ```
#[derive(Debug, Default)]
pub struct Irs<T> {
    context: Option<Context<T>>,
}

impl<T> Shuffler<T> for Irs<T> {
    fn shuffle<R>(&mut self, data: &mut Vec<T>, rng: &mut R) -> Result<(), &str>
    where
        T: Clone,
        R: rand::RngCore + ?Sized,
    {
        let mut context = self.get_reset_context(data);
        let mut initial_buffer = vec![0; 32];
        rng.fill_bytes(&mut initial_buffer);
        let mut rand_bit_iter = InfiniteBitIter::new(initial_buffer, rng);

        for _ in 0..128 {
            self.one_round(&mut context, data, &mut rand_bit_iter);
            if all_distinct(&context.bit_slots[..]) {
                self.context = Some(context);
                return Ok(());
            }
        }
        Err("bad randomness source")
    }
}

impl<T> Irs<T> {
    /// A function that allows us to reuse the old context
    /// with no new allocations as long as it may be safely used.
    ///
    /// If the old context can be reused, then this function will
    /// reinitialize it cheaply and move it out of the `Irs`.
    ///
    /// If it cannot be used, then this function will create a new
    /// context and set the internal `context` to `None`.
    ///
    /// The function does not return a mutable reference to make
    /// the borrow checker easier to deal with (as returning the
    /// reference would create a long-lived mutable reference to `*self`,
    /// which is bad).
    ///
    /// This behavior is kinda lame, so it may change in the future.
    fn get_reset_context(&mut self, data: &[T]) -> Context<T>
    where
        T: Clone,
    {
        match &mut self.context {
            Some(c) => {
                if data.len() != c.data_cp.len() {
                    self.context = Some(Context::new(data));
                } else {
                    c.data_cp.as_mut_slice().clone_from_slice(data);
                    c.bit_slots.iter_mut().for_each(|c| *c = 0);
                }
            }
            None => {
                self.context = Some(Context::new(data));
            }
        };
        self.context.take().unwrap()
    }

    fn one_round<R>(
        &self,
        ctx: &mut Context<T>,
        data: &mut Vec<T>,
        rand_bit_iter: &mut InfiniteBitIter<R>,
    ) where
        T: Clone,
        R: rand::RngCore + ?Sized,
    {
        let mut odd_count = 0;
        for slot in ctx.bit_slots.iter_mut() {
            *slot = (*slot << 1) | (rand_bit_iter.next_bit() as u64);
            odd_count += (*slot & 1) as usize;
        }

        let mut odd_moved = 0;
        let mut even_moved = 0;

        let size = data.len();

        for i in (0..size).rev() {
            let current = ctx.bit_slots[i];
            let position = if current % 2 == 1 {
                odd_moved += 1;
                size - odd_moved
            } else {
                even_moved += 1;
                size - odd_count - even_moved
            };
            ctx.bit_slots_cp[position] = current;
            ctx.data_cp[position] = data[i].clone();
        }

        core::mem::swap(&mut ctx.data_cp, data);
        core::mem::swap(&mut ctx.bit_slots_cp, &mut ctx.bit_slots);
    }
}

/// Tests whether all elements in a sorted slice are unique.
///
/// For a sorted slice this can be done in linear time, by
/// comparing each two consequtive items in the slice.
///
fn all_distinct<T>(sorted_data: &[T]) -> bool
where
    T: Eq,
{
    !sorted_data.windows(2).any(|s| s[0] == s[1])
}

#[cfg(test)]
mod tests {
    use super::*;


    struct DummyRng(u64);

    impl rand::RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            self.next_u64() as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(1);
            self.0
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.next_u64() as u8;
            }
        }
    }


    #[test]
    fn test_irs() {
        let mut irs = Irs::default();
        let mut rng = DummyRng(0);
        let input_data = vec![1, 2, 3, 4];
        let mut target = input_data.clone();
        irs.shuffle(&mut target, &mut rng).unwrap();
        assert_eq!(target.len(), input_data.len());
        assert_ne!(target, input_data);
        assert!(target.iter().all(|n| input_data.contains(n)));
    }

    #[test]
    fn test_get_reset_context() {
        let mut irs = Irs::default();

        let input_data = vec![1, 2, 3, 4];
        // we don't want any context if there was no data yet.
        assert_eq!(irs.context, None);
        let context = irs.get_reset_context(&input_data);
        assert_eq!(context.data_cp.len(), input_data.len());
        assert!(context.bit_slots.iter().all(|s| *s == 0));
    }

    #[test]
    fn test_all_distinct() {
        assert!(all_distinct(&[1, 2, 3, 4]));
        assert!(!all_distinct(&[1, 1, 2, 3]));
    }
}
