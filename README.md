# genetic-algo-rs

Following along with the Shorelark genetic algorithm tutorial

Things I've learnt:

- Marking things `pub(crate)` makes them `pub` but only within the current crate, which is handy for writing tests that access private member variables of structs, e.g.:
```rust
// In one file in your crate
pub struct MyStruct {
    pub(crate) value: u8,
}

// In another file in your crate
let s = MyStruct{value: 1};
println!("s.value == {}", s.value); // Without pub(crate) this wouldn't work
```

- The `rand_chacha` crate is handy for mocking up consistent, predictable "random" numbers in tests, e.g.:
```rust
use rand_chacha::ChaCha8Rng;

// Use the default seed...
let mut prng = ChaCha8Rng::from_seed(Default::default());

// ...or specify one
let mut prng = ChaCha8Rng::seed_from_u64(42);
```

- The `approx` crate is handy for comparing floating point numbers, e.g.:
```rust

let f1 = 100.0 / 3.0; // Float precision errors mean this might produce something like 33.333333333333336, so a direct assert_eq!() might fail
let f2 = 33.3333333333333;
approx::assert_relative_eq!(f1, f2);

// BUT you can't directly use it to compare, e.g. two Vec<f32>, instead it expects a slice, e.g.:
let vf1 = vec![1.0, 2.0];
let vf2 = vec![1.0, 2.0];
//approx::assert_relative_eq!(vf1, vf2); // ERROR: Won't compile!
approx::assert_relative_eq!(vf1.as_slice(), vf2.as_slice()); // This works though!

// This is different to the normal assert_eq!() macro which WILL let you directly compare those two vectors (and it works on the slices too)
```
