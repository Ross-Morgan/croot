# Croot

![Build](https://img.shields.io/github/actions/workflow/status/Ross-Morgan/croot/rust.yml)
![No Std Label](https://img.shields.io/badge/no--std-yes-blue)

> A Rust library for finding complex and principal roots of real and complex values;

# Context

Any number, real or complex, has n nth-roots.

For example, there are 4 values for the 4th-root of 1

Usually, we ignore all but the principal root; that with the largest real component

The principal 4th-root of 1 is `1`, but the others are `[-1, i, -i]`

# Examples

- [Principal Roots](#principal)
    - [Real](#real-principal)
    - [Complex](#real-complex)
- [All Roots](#all-roots)
    - [Real](#all-real)
    - [Complex](#all-complex)
- [Roots of Unity](#roots-of-unity)


## Principal

> The root with the largest real, and positive imaginary component

### _Real principal_

For finding the principal root of a real value, we use `principal_root`

```rust
```

### _Complex principal_

For finding the principal root of a complex value, we use `complex_principal_root`

```rust

```

## All Roots

> n values which when raised to the nth-power, give the original value

### _All Real_

For finding all roots of a real value, we use `root`

```rust
root(1.0, 2);  // [1.0, -1.0]
root(1.0, 4);  // [1.0, -1.0, i, -i]
root(81.0, 4); // [3.0, -3.0, 3i, -3i]
```
### _All Complex_

For finding all roots of a complex number, we use `complex_root`

```rust
let c1 = Complex64::new(3.0, 4.0)  // 3 + 4i
let c2 = Complex64::new(10.0, 2.0) // 10 + 2i

complex_root(c1, 3) // [1.6289 + 0.5202i, -1.2650 + 1.1506i, -0.3640 - 1.6708i]
complex_root(c2, 3) // [2.1639 + 0.14259, -1.2054 + 1.8027i, -0.9585 - 1.9453i]
```

## Roots of Unity

The nth-roots of unity are the nth-roots of `1`

The nth-roots of `1` can be found with `roots_of_unity`

```rust
roots_of_unity(3); // [1, -0.5 + 0.8660i, -0.5 - 0.8660i]
roots_of_unity(4); // [1, i, -1, -i]
```