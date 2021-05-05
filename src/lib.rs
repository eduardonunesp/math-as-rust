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

fn dot(a: Vec<i64>, b: Vec<i64>) -> i64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let rx = a[1] * b[2] - a[2] * b[1];
    let ry = a[2] * b[0] - a[0] * b[2];
    let rz = a[0] * b[1] - a[1] * b[0];
    vec![rx, ry, rz]
}

fn iter_sum(n: i64) -> i64 {
    (0..n).into_iter().sum()
}

fn iter_sum_2(n: i64) -> i64 {
    (0..n).map(|k| 2 * k + 1).into_iter().sum()
}

fn sum_to_n(n: f64) -> f64 {
    (n * (n + 1.)) / 2.
}

fn times(x: i64, y: i64) -> i64 {
    x * y
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
    fn test_dot() {
        let k = vec![0, 1, 0];
        let j = vec![1, 0, 0];
        let result = dot(k, j);
        assert!(result == 0);
    }

    #[test]
    fn test_cross() {
        let k = vec![0, 1, 0];
        let j = vec![1, 0, 0];
        let result = cross(k, j);
        assert!(result == vec![0, 0, -1]);
    }

    #[test]
    fn test_iter_sum() {
        assert!(iter_sum(100) == 4950)
    }

    #[test]
    fn test_iter_sum_2() {
        assert!(iter_sum_2(100) == 10000);
    }

    #[test]
    fn test_iter_sum_3() {
        let result = (1..3i32)
            .map(|i| (4..7i32).map(|j| 3 * i * j).sum::<i32>())
            .sum::<i32>();
        assert!(result == 135);
    }

    #[test]
    fn test_sum_to_n() {
        assert!(sum_to_n(100.) == 5050.)
    }

    #[test]
    fn test_capital_i() {
        let result = (1..7).into_iter().fold(1, |v, acc| acc * v);
        assert!(result == 720);
    }
}
