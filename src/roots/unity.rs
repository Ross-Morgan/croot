use num_complex::Complex64;

use super::roots::root;

pub fn roots_of_unity(nth_root: usize) -> Vec<Complex64> {
    root(1.0, nth_root)
}
