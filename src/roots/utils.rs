/// Returns the r value for any nth-root of a complex number
pub fn radius(n: f64, nth_root: usize) -> f64 {
    round(n.abs().powf(1.0 / (nth_root as f64)), 12)
}

/// Returns the nth-roots of 1

/// Round a number to a specified number of decimal places
///
/// # Panics
/// Will panic if to_round * 10^precision > f64::MAX
pub(super) fn round(to_round: f64, precision: i32) -> f64 {
    to_round - (to_round % 10.0_f64.powi(-precision))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_works() {
        let n = 10.3021;

        let a = round(n, 1);
        let b = round(n, 2);
        let c = round(n, 3);

        assert_eq!(a, 10.3);
        assert_eq!(b, 10.30);
        assert_eq!(c, 10.302);
    }

    #[test]
    fn radius_works() {
        let n1 = 1.0;
        let n2 = 81.0;
        let n3 = 6561.0;

        assert_eq!(radius(n1, 4), 1.0);
        assert_eq!(radius(n2, 4), 3.0);
        assert_eq!(radius(n3, 4), 9.0);
    }

    #[test]
    fn radius_sign_works() {
        let n1 = 1.0;
        let n2 = 81.0;
        let n3 = 6561.0;

        assert_eq!(radius(n1, 4), radius(-n1, 4));
        assert_eq!(radius(n2, 4), radius(-n2, 4));
        assert_eq!(radius(n3, 4), radius(-n3, 4));
    }
}
