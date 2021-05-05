#![allow(warnings, unused)]

use std::convert::TryFrom;

use num::Complex;

#[macro_use]
extern crate is_close;

fn is_almost_equal(x: f64, y: f64, epsilon: f64) -> bool {
    (x - y).abs() < (10f64.powf(-epsilon))
}

fn plus(x: i32, y: i32) -> i32 {
    x + y
}

fn print_sqrt() {
    println!("{}", 2f64.sqrt());
}

fn complex_number() {
    let complex_integer = num::Complex::new(1, 1);
    println!("{}", complex_integer);
}

fn complex_sqrt() {
    let complex_integer = num::Complex::new(1f64, 1f64);
    println!("{}", complex_integer.sqrt());
}

fn multiply(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let it = a.iter().zip(b.iter());
    it.map(|(x, y)| x * y).collect()
}

fn multiply_scalar(a: Vec<i64>, scalar: i64) -> Vec<i64> {
    a.iter().map(|v| v * scalar).collect()
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
    fn test_is_almost_equal() {
        // we gave it a the tolerance we want, 5 decimal places.
        assert!(is_almost_equal(std::f64::consts::PI, 3.14159, 1e-5))
    }

    #[test]
    fn test_plus() {
        assert!(
            plus(1, 1) == 2,
            "DANGER: PHYSICS IS BROKEN. PLEASE STAY INSIDE. "
        )
    }

    #[test]
    fn test_print_sqrt() {
        print_sqrt();
    }

    #[test]
    fn test_complex() {
        complex_number();
    }

    #[test]
    fn test_complex_sqrt() {
        complex_sqrt();

        let c1 = num::complex::Complex::new(-1, 0);
        let c2 = num::complex::Complex::new(0, 1);
        assert!(c1 != c2);
    }

    #[test]
    fn test_multiply_vectors() {
        let s = 3;
        let k = vec![1, 2];
        let j = vec![2, 3];
        let tmp = multiply(k, j);
        assert!(multiply_scalar(tmp, s) == vec![6, 18]);
    }

    #[test]
    fn test_sum_to_n() {
        assert!(sum_to_n(100.) == 5050.)
    }
}
