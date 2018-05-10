extern crate num;
use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

#[allow(dead_code)]
fn square_add_loop(c: f64) {
    let mut x = 0.;
    loop {
        x = x * x + c;
    }
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/// Try to determine if 'c' is in the Mandelbrot set, using at most 'limit'
/// iterations to decide.alloc
///
/// If 'c' is not a member, return 'Some(i)', where 'i' is the number of
/// iterations it took for 'c' to leave the circle of radius two centered on the
/// origin. If 'c' seems to be a member (more precisely, if we reach the
/// iteration limit without being able to prove that 'c' is not a member),
/// return 'None'
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// Parse the string 's' as a coordinate pair, like '"400x600"' or '"1.0.0.5"'.
///
/// Specifically 's' should have the form <left><sep><right>, where <sep> is
/// the character given by the 'separator' argument, and <left> and <right> are
/// both strings that can  be parsed by 'T::from_str'.
///
/// If 's' has the proper form, return 'Some<(x, y)>'. If it doesn't parse
/// correctly, return 'None'.
fn parse_pair<T: FromStr>(s: &str, seperator: char) -> Option<(T, T)> {
    match s.find(seperator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

/// Parse a pair of floating-point numbers separated by a comma as a complex
/// number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im}),
        None => None
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
        Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}