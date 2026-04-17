use std::{f64::consts::PI, vec};

use num_complex::{Complex, Complex64};

fn main() {
    println!("Hello, world!");
}

// this will be our main function
// We should have it return the DFT vector so we can print it out in our main function
fn fft() {

}

// this is the twiddle factor helper
// Takes in a pair of vectors of the pre seperated odd / even indexes,
// Returns twiddle factors with first being odds and seond being evens
fn twiddle_factor(oddsevens: (Vec<f64>, Vec<f64>)) -> (Vec<Complex64>, Vec<Complex64>) {
    let odds: Vec<f64> = oddsevens.0;
    let evens: Vec<f64> = oddsevens.1;
    let mut twiddleodds: Vec<Complex64> = vec![];
    let mut twiddleevens: Vec<Complex64> = vec![];
    for k in 0..odds.len() {
        let realcomp: f64 = (2.0 * PI * k as f64) / odds.len() as f64;
        twiddleodds.push(Complex { re: realcomp.cos() , im: -realcomp.sin() });
    }
    for k in 0..evens.len() {
        let realcomp: f64 = (2.0 * PI * k as f64) / evens.len() as f64;
        twiddleevens.push(Complex { re: realcomp.cos() , im: -realcomp.sin() });
    }
    return (twiddleodds, twiddleevens);
}

// this is the bit reversal
fn bit_reversal() {

}