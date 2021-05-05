#![allow(warnings, unused)]

#[macro_use]
extern crate is_close;

fn is_almost_equal(x: f64, y: f64, epsilon: f64) -> bool {
    (x - y).abs() < (10f64.powf(-epsilon))
}

fn plus(x: i32, y: i32) -> i32 {
    x + y
}

fn sum_to_n(n: f64) -> f64 {
    (n * (n + 1.)) / 2.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_close() {
        // isclose doesn't have a third argument for tolerance, so this is false
        assert_ne!(is_close!(std::f64::consts::PI, 3.14159), true);
    }

    #[test]
    fn test_plus() {
        assert!(
            plus(1, 1) == 2,
            "DANGER: PHYSICS IS BROKEN. PLEASE STAY INSIDE. "
        )
    }

    #[test]
    fn test_is_almost_equal() {
        // we gave it a the tolerance we want, 5 decimal places.
        assert!(is_almost_equal(std::f64::consts::PI, 3.14159, 1e-5))
    }

    #[test]
    fn test_sum_to_n() {
        assert!(sum_to_n(100.) == 5050.)
    }
}
