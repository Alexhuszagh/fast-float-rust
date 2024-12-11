# fast-float2

[![Build](https://github.com/Alexhuszagh/fast-float-rust/workflows/CI/badge.svg)](https://github.com/Alexhuszagh/fast-float-rust/actions?query=branch%3Amaster)
[![Latest Version](https://img.shields.io/crates/v/fast-float2.svg)](https://crates.io/crates/fast-float2)
[![Documentation](https://docs.rs/fast-float2/badge.svg)](https://docs.rs/fast-float2)
[![Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rustc 1.37+](https://img.shields.io/badge/rustc-1.37+-lightgray.svg)](https://blog.rust-lang.org/2019/08/15/Rust-1.37.0.html)

This crate provides a super-fast decimal number parser from strings into floats.

```toml
[dependencies]
fast-float2 = "0.2.3"
```

There are no dependencies and the crate can be used in a no_std context by disabling the "std" feature.

*Compiler support: rustc 1.37+.*

This crate is in maintenance mode for bug fixes (especially security patches): minimal feature enhancements will be accepted. This implementation has been adopted by the Rust standard library: if you do not need parsing directly from bytes and/or partial parsers, you should use [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) for [f32](https://doc.rust-lang.org/std/primitive.f32.html) or [f64](https://doc.rust-lang.org/std/primitive.f64.html) instead.

## Usage

There's two top-level functions provided:
[`parse()`](https://docs.rs/fast-float/latest/fast_float/fn.parse.html) and
[`parse_partial()`](https://docs.rs/fast-float/latest/fast_float/fn.parse_partial.html), both taking
either a string or a bytes slice and parsing the input into either `f32` or `f64`:

- `parse()` treats the whole string as a decimal number and returns an error if there are
  invalid characters or if the string is empty.
- `parse_partial()` tries to find the longest substring at the beginning of the given input
  string that can be parsed as a decimal number and, in the case of success, returns the parsed
  value along the number of characters processed; an error is returned if the string doesn't
  start with a decimal number or if it is empty. This function is most useful as a building
  block when constructing more complex parsers, or when parsing streams of data.

Example:

```rust
// Parse the entire string as a decimal number.
let s = "1.23e-02";
let x: f32 = fast_float2::parse(s).unwrap();
assert_eq!(x, 0.0123);

// Parse as many characters as possible as a decimal number.
let s = "1.23e-02foo";
let (x, n) = fast_float2::parse_partial::<f32, _>(s).unwrap();
assert_eq!(x, 0.0123);
assert_eq!(n, 8);
assert_eq!(&s[n..], "foo");
```

## Details

This crate is a direct port of Daniel Lemire's [`fast_float`](https://github.com/fastfloat/fast_float)
C++ library (valuable discussions with Daniel while porting it helped shape the crate and get it to
the performance level it's at now), with some Rust-specific tweaks. Please see the original
repository for many useful details regarding the algorithm and the implementation.

The parser is locale-independent. The resulting value is the closest floating-point values (using either
`f32` or `f64`), using the "round to even" convention for values that would otherwise fall right in-between
two values. That is, we provide exact parsing according to the IEEE standard.

Infinity and NaN values can be parsed, along with scientific notation.

Both little-endian and big-endian platforms are equally supported, with extra optimizations enabled
on little-endian architectures.

Since [fast-float-rust](https://github.com/aldanor/fast-float-rust) is unmaintained, this is a fork
containing the patches and security updates.

## Testing

There are a few ways this crate is tested:

- A suite of explicit tests (taken from the original library) covering lots of edge cases.
- A file-based test suite (taken from the original library; credits to Nigel Tao), ~5M tests.
- All 4B float32 numbers are exhaustively roundtripped via ryu formatter.
- Roundtripping a large quantity of random float64 numbers via ryu formatter.
- Roundtripping float64 numbers and fuzzing random input strings via cargo-fuzz.
- All explicit test suites run on CI; roundtripping and fuzzing are run manually.

## Performance

The presented parser seems to beat all of the existing C/C++/Rust float parsers known to us at the
moment by a large margin, in all of the datasets we tested it on so far – see detailed benchmarks
below (the only exception being the original fast_float C++ library, of course – performance of
which is within noise bounds of this crate). On modern machines like Apple M1, parsing throughput
can reach up to 1.5 GB/s.

While various details regarding the algorithm can be found in the repository for the original
C++ library, here are few brief notes:

- The parser is specialized to work lightning-fast on inputs with at most 19 significant digits
  (which constitutes the so called "fast-path"). We believe that most real-life inputs should
  fall under this category, and we treat longer inputs as "degenerate" edge cases since it
  inevitable causes overflows and loss of precision.
- If the significand happens to be longer than 19 digits, the parser falls back to the "slow path",
  in which case its performance roughly matches that of the top Rust/C++ libraries (and still
  beats them most of the time, although not by a lot).
- On little-endian systems, there's additional optimizations for numbers with more than 8 digits
  after the decimal point.

## Benchmarks

Below are tables of best timings in nanoseconds for parsing a single number
into a 64-bit float (using the median score).

### Intel i7-14700K

Intel i7-14700K 3.40GHz, Linux (WSL2), Rust 1.81.

|                        | `canada` | `mesh`   | `uniform` | `bi`  | `iei`  | `rec32` |
| ---------------------- | -------- | -------- | --------- | ----- | ------ | ------- |
| fast-float2            | 9.98     | 5.56     | 10.08     | 56.19 | 14.52  | 15.09   |
| fast-float             | 9.77     | 5.04     | 9.05      | 57.52 | 14.40  | 14.23   |
| lexical                | 10.62    | 4.93     | 9.92      | 26.40 | 12.43  | 14.40   |
| from_str               | 11.59    | 5.92     | 11.23     | 35.92 | 14.75  | 16.76   |
| fast_float (C++)       | 12.58    | 6.35     | 11.86     | 31.55  | 12.22  | 11.97   |
| abseil (C++)           | 25.32    | 15.70    | 25.88     | 43.42  | 23.54  | 26.75   |
| netlib (C)             | 35.10    | 10.22    | 37.72     | 68.63  | 23.07  | 38.23   |
| strtod (C)             | 52.63    | 26.47    | 46.51     | 88.11  | 33.37  | 53.36   |
| doubleconversion (C++) | 32.50    | 14.69    | 47.80     | 70.01  | 205.72 | 45.66   |

Note that the random number generation seems to differ between C/C++ and Rust, since the Rust implementations are slightly faster for pre-determined datasets like `canada` and `mesh`, but equivalent random number generators are slightly slower. Any performance penalty with `fast-float2` occurred due to fixing the UB in [check_len](https://github.com/aldanor/fast-float-rust/issues/28). The massive performance differences between `fast-float` (Rust) and `fast_float` (C++) are expected due to a faster fallback algorithms ([#96](https://github.com/fastfloat/fast_float/pull/96) and [#104](https://github.com/fastfloat/fast_float/pull/104)) used in these cases.

#### Parsers

- `fast-float2` - this very crate
- `fast-float` - the pre-ported variant
- `lexical` – `lexical_core`, v1.0.05
- `from_str` – Rust standard library, `FromStr` trait
- `fast_float (C++)` – original C++ implementation of 'fast-float' method
- `abseil (C++)` – Abseil C++ Common Libraries
- `netlib (C++)` – C++ Network Library
- `strtod (C)` – C standard library

#### Datasets

- `canada` – numbers in `canada.txt` file
- `mesh` – numbers in `mesh.txt` file
- `uniform` – uniform random numbers from 0 to 1
- `bi` – large, integer-only floats <!-- `big_ints` -- >
- `int_e_int` – random numbers of format `%de%d` <!-- `int_e_int` -->
- `rec32` – reciprocals of random 32-bit integers <!-- `one_over_rand32` -->

#### Notes

- The two test files referred above can be found in
[this](https://github.com/lemire/simple_fastfloat_benchmark) repository.
- The Rust part of the table (along with a few other benchmarks) can be generated via
  the benchmark tool that can be found under `extras/simple-bench` of this repo.
- The C/C++ part of the table (along with a few other benchmarks and parsers) can be
  generated via a C++ utility that can be found in
  [this](https://github.com/lemire/simple_fastfloat_benchmark) repository.

<br>

#### References

- Daniel Lemire, [Number Parsing at a Gigabyte per Second](https://arxiv.org/abs/2101.11408), Software: Practice and Experience 51 (8), 2021.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
