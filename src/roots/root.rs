use alloc::vec::{self, Vec};
use core::f64::consts::PI;

use num_complex::Complex64;

use super::unity::roots_of_unity;
use super::utils::radius;

/// Returns nth-roots of n
///
/// # Panics
/// Panics if `nth_root == 0`
///
/// # Examples
///
/// ```
/// # use num_complex::Complex64;
/// # use croot::prelude::*;
///
/// // Find roots and use precision
/// // to remove floating-point errors
/// let roots = root(81.0, 4).precision(10);
///
/// assert!(roots.contains(&Complex64::new(3.0, 0.0)));
/// assert!(roots.contains(&Complex64::new(0.0, 3.0)));
/// assert!(roots.contains(&Complex64::new(-3.0, 0.0)));
/// assert!(roots.contains(&Complex64::new(0.0, -3.0)));
/// ```
pub fn root(n: f64, nth_root: usize) -> Vec<Complex64> {
    if n > 0.0 {
        // Multiply each root of unity by radius length
        roots_of_unity(nth_root).into_iter().map(|c| c * n).collect()
    } else if n < 0.0 {
        //TODO Implement seperately?
        complex_root(Complex64::new(n, 0.0), nth_root)
    } else {
        vec![0; nth_root]
    }
}


/// Returns nth-roots of c
///
/// # Panics
/// Panics if `nth_root == 0`
pub fn complex_root(c: Complex64, nth_root: usize) -> Vec<Complex64> {
    if nth_root == 0 {
        panic!("Root cannot be zero");
    }

    let principal_theta = c.arg() / (nth_root as f64);
    let real = radius(c.norm(), nth_root);

    (0..nth_root)
        .map(|idx| {    
            let n_theta = principal_theta + (2.0 * PI * (idx as f64) / (nth_root as f64));
            Complex64::cis(n_theta) * real
        })
        .collect()
}
