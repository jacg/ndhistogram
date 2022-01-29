extern crate ndhistogram;
use ndhistogram::axis::Uniform;
use ndhistogram::{ndhistogram, Histogram};

fn main() {
    let mut hist = ndhistogram!(Uniform::with_step_size(11, -5, 1); i8);

    for x in -5..=5 {
        hist.fill_with(&x, &(x * x));
    }
    println!("{hist}")
}
