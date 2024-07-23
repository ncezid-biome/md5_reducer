extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use num::Num;
use clap::Parser;

/// Command-line arguments parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The MD5 hash to be reduced
    #[arg(short, long)]
    md5: Vec<String>,

    /// The maximum number of bits for the reduced hash
    #[arg(short, long)]
    bits: u32,

    /// Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool,
}

/// Reduces an MD5 hash to a number with a maximum number of bits
fn reduce_md5(md5_hex: &str, max_bits_in_result: u32) -> BigUint {
    let p = (BigUint::one() << max_bits_in_result) - BigUint::one();
    let mut rest = BigUint::from_str_radix(md5_hex, 16).unwrap();
    let mut result = BigUint::zero();

    while !rest.is_zero() {
        result ^= &rest & &p;
        rest >>= max_bits_in_result;
    }

    result
}

/// Reduces a list of MD5 hashes to numbers with a maximum number of bits
fn reduce_all_md5s(md5_hex: &Vec<String>, max_bits_in_result: u32) -> Vec<BigUint> {
    md5_hex.iter().map(|md5| 
        reduce_md5(md5, max_bits_in_result))
        .collect()
}

fn main() {
    let args = Args::parse();
/*
    if args.verbose {
        println!("MD5: {}", args.md5);
        println!("Max bits: {}", args.bits);
    }
*/
    let reduced_hashes = reduce_all_md5s(&args.md5, args.bits);

    for (i, hash) in reduced_hashes.iter().enumerate() {
        println!("{}\t{}", i, hash);
    }
}

/// A function to test the speed of the reduce_md5 function
#[test]
fn test_speed() {
    let md5 = "d41d8cd98f00b204e9800998ecf8427e";
    let max_bits = 56;
    let iterations = 1000000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        reduce_md5(md5, max_bits);
    }
    let duration = start.elapsed();

    eprintln!("==== {} iterations in {:?}", iterations, duration);
}
