use alloc::vec::Vec;
use core::f64::consts::PI;

use num_complex::Complex64;

///
pub fn real_root(n: f64, nth_root: usize) -> f64 {
    round(n.abs().powf(1.0 / (nth_root as f64)), 12)
}

/// Returns nth-roots of n
///
/// # Panics
/// Panics if `nth_root == 0`
///
/// # Examples
///
/// ```
/// # use num_complex::Complex64;
///
/// // Find roots and use precision
/// // to remove floating-point errors
/// let roots = root(81.0, 4).precision(10);
///
/// assert!(roots.contains(Complex64::new(3.0, 0.0)));
/// assert!(roots.contains(Complex64::new(0.0, 3.0)));
/// assert!(roots.contains(Complex64::new(-3.0, 0.0)));
/// assert!(roots.contains(Complex64::new(0.0, -3.0)));
/// ```
pub fn root(n: f64, nth_root: usize) -> Vec<Complex64> {
    if nth_root == 0 {
        panic!("Root cannot be zero");
    }

    // Part to add to theta for negative roots
    let add_part = (1.0 - n.signum()) / 2.0;
    let real = real_root(n, nth_root);

    (0..nth_root)
        .map(|idx| {
            let theta = (2.0 * (idx as f64) + add_part) / (nth_root as f64) * PI;

            Complex64::cis(theta) * real
        })
        .collect()
}

/// Returns the nth-roots of 1
pub fn roots_of_unity(nth_root: usize) -> Vec<Complex64> {
    root(1.0, nth_root)
}

/// Returns the principal root of n
///
/// The principal root is that with the largest real component
///
/// If real components of 2 roots are equal,
/// the root with the largest complex component is returned
pub fn principal_root(n: f64, nth_root: usize) -> Complex64 {
    if n == 0.0 {
        Complex64::default()
    } else {
        root(n, nth_root)
            .into_iter()
            .reduce(|lhs, rhs| {
                if lhs.re >= rhs.re {
                    lhs
                } else if lhs.re == rhs.re {
                    if lhs.im >= rhs.im {
                        lhs
                    } else {
                        rhs
                    }
                } else {
                    rhs
                }
            })
            .expect("root function returned no roots")
    }
}

/// Round a number to a specified number of decimal places
///
/// # Panics
/// Will panic if to_round * 10^precision > f64::MAX
///
/// TODO: Experiment with rounding using logarithms
fn round(to_round: f64, precision: i32) -> f64 {
    (to_round * (10.0f64.powi(precision))).round() / (10.0f64.powi(precision))
}

pub mod prelude {
    pub use super::{principal_root, real_root, root, roots_of_unity};
}
