use alloc::vec::Vec;
use core::f64::consts::PI;

use num_complex::Complex64;

/// Returns nth-roots of n
pub fn root(n: f64, nth_root: usize) -> Vec<Complex64> {
    (0..nth_root).map(|idx| {
        let theta = 2.0 * (idx as f64) / (nth_root as f64) * PI;
        Complex64::from_polar(n, theta)
    }).collect()
}

/// Returns the nth-roots of 1
pub fn roots_of_unity(nth_root: usize) -> Vec<Complex64> {
    root(1.0, nth_root)
}

/// Returns the principal root of n
///
/// The principal root is that with the largest real component
pub fn principal_root(n: f64, nth_root: usize) -> Complex64 {
    root(n, nth_root)
        .into_iter()
        .reduce(|lhs, rhs| {
            if lhs.re >= rhs.re {
                lhs
            } else {
                rhs
            }
        })
        .unwrap_or_default()
}

pub mod prelude {
    pub use super::{principal_root, root, roots_of_unity};
}
