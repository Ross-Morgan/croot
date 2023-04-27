use alloc::vec::Vec;
use core::f64::consts::PI;

use num_complex::Complex64;

pub fn roots_of_unity(nth_root: usize) -> Vec<Complex64> {
    (0..nth_root)
        .map(|k| Complex64::cis(2.0 * (k as f64) * PI / (nth_root as f64)))
        .collect()
}
