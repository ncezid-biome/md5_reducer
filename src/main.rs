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
    md5: String,

    /// The maximum number of bits for the reduced hash
    #[arg(short, long)]
    bits: u32,

    /// Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool,
}

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

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("MD5: {}", args.md5);
        println!("Max bits: {}", args.bits);
    }

    let reduced_hash = reduce_md5(&args.md5, args.bits);

    println!("Reduced MD5: {}", reduced_hash);
}
