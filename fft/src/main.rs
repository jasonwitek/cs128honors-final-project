use std::{f64::consts::PI, vec};

use num_complex::{Complex, Complex64};

fn main() {
    println!("Hello, world!");
    // Make way to input data?
    // Maybe read from csv file if possible
}

// this will be our main function
// We should have it return the DFT vector so we can print it out in our main function
fn fft(input_array: Vec<f64>) -> Vec<Complex64> {
    let odds_evens = splitarray(input_array);
    let twiddle_factors: (Vec<Complex64>, Vec<Complex64>) = twiddle_factor((odds_evens.0, odds_evens.1));
    todo!();
}

// this is the twiddle factor helper
// Takes in a pair of vectors of the pre seperated odd / even indexes,
// Returns twiddle factors with first being odds and seond being evens
fn twiddle_factor(oddsevens: (Vec<f64>, Vec<f64>)) -> (Vec<Complex64>, Vec<Complex64>) {
    let odds: Vec<f64> = oddsevens.0;
    let evens: Vec<f64> = oddsevens.1;
    let mut twiddle_odds: Vec<Complex64> = vec![];
    let mut twiddle_evens: Vec<Complex64> = vec![];
    for k in 0..odds.len() {
        let realcomp: f64 = (2.0 * PI * k as f64) / odds.len() as f64;
        twiddle_odds.push(Complex { re: realcomp.cos() , im: -realcomp.sin() });
    }
    for k in 0..evens.len() {
        let realcomp: f64 = (2.0 * PI * k as f64) / evens.len() as f64;
        twiddle_evens.push(Complex { re: realcomp.cos() , im: -realcomp.sin() });
    }
    return (twiddle_odds, twiddle_evens);
}

// This is a temporary splitter function while we work on the iterative approach
fn splitarray(input_array: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let mut odds: Vec<f64> = vec![];
    let mut evens: Vec<f64> = vec![];
    for i in 0..input_array.len() {
        if i % 2 == 0 {
            evens.push(input_array[i]);
        } else {
            odds.push(input_array[i]);
        }
    }
    return (odds, evens);
}

// this is the bit reversal
fn bit_reversal() {

}

// Performs the buttery fly operation, first half of the array is for the evens, second half is for the odds
// Note how the even index twiddle factors are not used - they will be used in the recursive approach
fn butterflycomputation(twiddle_factors: (Vec<Complex64>, Vec<Complex64>), odds_evens: (Vec<f64>, Vec<f64>)) -> Vec<Complex64> {
    let mut computedbutterfly: Vec<Complex64> = vec![];
    for i in 0..twiddle_factors.0.len() {
        computedbutterfly.push(Complex { re: odds_evens.1[i], im: 0.0 }  + twiddle_factors.0[i] * Complex { re: odds_evens.0[i], im: 0.0 });
    }
    for i in 0..twiddle_factors.0.len() {
        computedbutterfly.push(Complex { re: odds_evens.1[i], im: 0.0 }  - twiddle_factors.0[i] * Complex { re: odds_evens.0[i], im: 0.0 });
    }
    return computedbutterfly;
}