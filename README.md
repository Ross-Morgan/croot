# Croot

![No Std Label](https://img.shields.io/badge/no--std-yes-blue?style=for-the-badge)
![Build](https://img.shields.io/github/actions/workflow/status/Ross-Morgan/croot/rust.yml?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/croot?style=for-the-badge)
![Crates.io (recent)](https://img.shields.io/crates/dr/croot?style=for-the-badge)

> A Rust library for finding complex and principal roots of real and complex values;

# Examples

## Real index

For finding roots where the index is a real number

Returns a vector containing all roots.

```rust
use croot::prelude::*;

let roots = root(16.0, 4).approx(10);

roots; // [4, -4, 4i, -4i]
```

## Complex index

For finding roots where the index is a complex number

Returns a complex number and a multiplicative period

```rust
use croot::prelude::*;
use num_complex::Complex64;

let index = Complex64::new(3.0, 4.0);
let (principal, period) = complex_root(10.0, index).approx(10);

principal; // first root
period; // number to multiply by to get next root
```
