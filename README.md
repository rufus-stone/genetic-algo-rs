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

let f1 = 100.0 / 3.0; // This might produce 33.333333333333336
let f2 = 33.3333333333333;
approx::assert_relative_eq!(f1, f2);
```

- asfd