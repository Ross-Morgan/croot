use num_complex::Complex64;

use super::utils::radius;

/// Returns the principal root of a real number n
///
/// The principal root is that with the largest real component
///
/// If real components of 2 roots are equal,
/// the root with the largest complex component is returned
pub fn principal_root(n: f64, nth_root: usize) -> Complex64 {
    complex_principal_root(Complex64::new(n, 0.0), nth_root)
}

/// Returns the principal root of a real number n
///
/// The principal root is that with the largest real component
///
/// If real components of 2 roots are equal,
/// the root with the largest complex component is returned
pub fn complex_principal_root(c: Complex64, nth_root: usize) -> Complex64 {
    if c == Complex64::new(0.0, 0.0) {
        c
    } else if c.re >= 0.0 && c.im == 0.0 {
        let r = radius(c.norm(), nth_root);

        Complex64::new(r, 0.0)
    } else {
        let principal_theta = c.arg() / (nth_root as f64);

        Complex64::cis(principal_theta) * radius(c.norm(), nth_root)
    }
}
