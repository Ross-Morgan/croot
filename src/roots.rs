use alloc::vec::Vec;
use core::f64::consts::PI;

use num_complex::Complex64;

/// Gives all real and complex roots of any given value
pub fn roots_of(n: f64, nth_root: usize) -> Vec<Complex64> {
    (0..nth_root).map(|idx| {
        let theta = 2.0 * (idx as f64) / (nth_root as f64) * PI;
        Complex64::from_polar(n, theta)
    }).collect()
}

pub fn roots_of_unity(nth_root: usize) -> Vec<Complex64> {
    roots_of(1.0, nth_root)
}

pub fn principal_root(n: f64, nth_root: usize) -> Complex64 {
    roots_of(n, nth_root)
        .into_iter()
        .max_by(|lhs, rhs| lhs.re.total_cmp(&rhs.re))
        .expect("No roots were returned")
}

pub mod prelude {
    pub use super::{roots_of, roots_of_unity};
}
