# md5_reducer

Makes MD5sums with fewer bits.
It could be useful for making smaller databases or somehow reducing your footprint.

## Installation

This uses Rust and Cargo and so the installation is

```bash
cargo build --release
# update the path or cp the executable to your existing path
export PATH=$PATH:$(realpath ./target/release)
# or 
cp -nv ./target/release/md5_reducer $HOME/bin/
```

## Usage

```text
Usage: md5_reducer [OPTIONS] --bits <BITS>

Options:
  -m, --md5 <MD5>    The MD5 hash to be reduced
  -b, --bits <BITS>  The maximum number of bits for the reduced hash
  -v, --verbose      Sets the level of verbosity
  -h, --help         Print help
  -V, --version      Print version
```

## Benchmark

Run the cargo test to run 1M hashes

```bash
cargo test
```

Which produces something like

```text
running 1 test
test test_speed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.43s
```
