use alloc::vec::Vec;

use num_complex::Complex64;

use super::root::root;

pub fn roots_of_unity(nth_root: usize) -> Vec<Complex64> {
    root(1.0, nth_root)
}
