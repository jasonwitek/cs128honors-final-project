use std::{f64::consts::PI, vec};
use std::io::{self};
use num_complex::{Complex, Complex64};

fn main() {
    println!("Enter in single-spaced array vlues (or one per line");
    println!("Type 'done' when finished");

    let input = read_input();
    if input.is_empty() {
        eprintln!("input must not be empty");
        return;
    }

    let n = input.len();

    if !n.is_power_of_two() {
        eprintln!("Input length must be power of two");
        return;
    }

    println!("\nRecursive FFT:");
    let result1 = fft_recursive(&input);
    for (i,val) in result1.iter().enumerate() {
        println!("X[{i}] = {:.4} + {:.4}j", val.re, val.im);
    }

    println!("\nIterative FFT:");
    let result2 = fft_iterative(input);
    for (i,val) in result2.iter().enumerate() {
        println!("X[{i}] = {:.4} + {:.4}j", val.re, val.im);
    }

    println!("\nHow to read the output:");
    println!(" Index 0 = sum of all values");
    println!(" Each complex number: magnitude = sqrt(re² + im²), phase = arctan(im, re)");
}

fn read_input() -> Vec<f64> {
    let mut values: Vec<f64> = vec![];

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Can't read line");

        let trim = line.trim();

        if trim.is_empty() {
            continue;
        }

        if trim.eq_ignore_ascii_case("done") {
            break;
        }

        for num in trim.split_whitespace() {
            match num.parse::<f64>() {
                Ok(val) => values.push(val),
                Err(_) => eprintln!("Invalid value, skipping: '{}'", num)
            }
        }
    }
    return values;
}

// this will be our main function
// We should have it return the DFT vector so we can print it out in our main function
fn fft_iterative(input_array: Vec<f64>) -> Vec<Complex64> {
    let n = input_array.len();
    assert!(n.is_power_of_two(), "Cannot perform Cooley Tukey method on vec of size not 2^n");

    let mut output: Vec<Complex64> = vec![];
    for i in 0..n {
        output.push(Complex { re: input_array[i], im: 0.0 });
    }

    bit_reversal(&mut output);

    let mut len = 2; // 2^1, will double every time
    while len <= n { // keep going till full length
        let twiddle = twiddle_factor(len);

        for block_start in (0..n).step_by(len) {
            for i in 0..len/2 {
                let even = output[block_start + i];
                let odd = twiddle[i] * output[block_start + i + len / 2];

                output[block_start + i] = even + odd;
                output[block_start + i + len / 2] = even - odd;
            }
        }
        len *= 2;
    }
    output
}

// Our recursive approach, recurses to base case (length of 1) then builds up FFT
fn fft_recursive(input_array: &[f64]) -> Vec<Complex64> {
    let n = input_array.len();
    assert!(n.is_power_of_two(), "Cannot perform Cooley Tukey method on vec of size not 2^n");
    if n == 1 {
        return vec![Complex::new(input_array[0], 0.0)];
    }

    let odds_evens = splitarray(input_array);

    let evens_fft = fft_recursive(&odds_evens.1);
    let odds_fft = fft_recursive(&odds_evens.0);

    let twiddle = twiddle_factor(n);

    butterflycomputation(twiddle, evens_fft, odds_fft)
}

// Splitter function for recursive function, 0=odds, 1=evens
fn splitarray(input_array: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut odds: Vec<f64> = vec![];
    let mut evens: Vec<f64> = vec![];
    for i in 0..input_array.len() {
        if i % 2 == 0 {
            evens.push(input_array[i]);
        } else {
            odds.push(input_array[i]);
        }
    }
    (odds, evens)
}

// this is the twiddle factor helper
// Rewritten since twiddle factors only depend on index, not value
fn twiddle_factor(n: usize) -> Vec<Complex64> {
    let mut twiddlers: Vec<Complex64> = vec![];
    for i in 0..n/2 {
        let angle: f64 = (2.0 * PI * i as f64) / n as f64;
        twiddlers.push(Complex { re: angle.cos(), im: -angle.sin() })
    }
    twiddlers
}

// Performs the butterfly computation, calculates the odd twiddles and then adds or subtracts it to the even one to get the corresponding DFT value
fn butterflycomputation(twiddle: Vec<Complex64>, even_fft: Vec<Complex64>, odd_fft: Vec<Complex64>) -> Vec<Complex64> {
    let n = even_fft.len() * 2;
    let mut computedbutterfly: Vec<Complex64> = vec![Complex {re: 0.0, im: 0.0}; n]; // Makes blank Complex64 size of n
    for i in 0..n/2 {
        let odd_twiddle = twiddle[i] * odd_fft[i];
        computedbutterfly[i] = even_fft[i] + odd_twiddle;
        computedbutterfly[i + n / 2] = even_fft[i] - odd_twiddle;
    }
    computedbutterfly
}

fn bit_reversal(a: &mut Vec<Complex64>) {
    let n = a.len();
    let log_n = n.trailing_zeros();

    for i in 0..n {
        // Compute j = bit-reverse of i using log_n bits
        let j = reverse_bits(i, log_n);

        if i < j {
            a.swap(i, j);
        }
    }

}

fn reverse_bits(mut num: usize, bit_count: u32) -> usize {
    let mut result: usize = 0;
    for _i in 0..bit_count {
        // Shift result left to make room for the next bit
        result <<= 1;

        // Take the lowest bit of num and put it into result
        // (num & 1) isolates the last bit: if num=5 (101), num&1 = 1
        result = result | (num & 1);

        // Shift num right to expose the next bit
        // After this, num=5 (101) becomes num=2 (10)
        num >>= 1;
    }
    result
}