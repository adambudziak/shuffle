# shuffle
Various shuffling algorithms for rust.

## Currently implemented shuffling algorithms
- [x] Inverse Riffle Shuffle
- [x] Fisher-Yates
- [ ] ... ? TODO

## `no_std` Support

This crate is `no_std` compatible but requires the `alloc` crate (for `Vec`).

## Examples

```
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

let mut rng = StepRng::new(2, 13);
let mut irs = Irs::default();
let mut input = vec![1, 2, 3, 4, 5];

irs.shuffle(&mut input, &mut rng);
assert_eq!(&input, &[4, 1, 5, 3, 2]);
```
