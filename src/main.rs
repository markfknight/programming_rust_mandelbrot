extern crate num;
use num::Complex;

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