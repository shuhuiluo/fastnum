# fastnum

[![doc.rs](https://img.shields.io/docsrs/fastnum)](https://docs.rs/fastnum/latest/fastnum)
[![Crates.io](https://img.shields.io/crates/d/fastnum?logo=rust)](https://crates.io/crates/fastnum)

Fixed-size signed and unsigned integers and arbitrary precision decimal numbers implemented in pure Rust. Suitable for
financial, crypto and any other fixed-precision calculations.

[API Docs](https://docs.rs/fastnum/latest/fastnum)

## Overview

This crate is inspired by [num_bigint](https://docs.rs/num-bigint/0.4.6/num_bigint/)
and [bigdecimal](https://docs.rs/bigdecimal/latest/bigdecimal/) - an amazing crates that allows you to store big
integers and arbitrary precision fixed-point decimal numbers almost any precision.

[BigInt](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html) internally uses a [`Vec`] of decimal digits
the size of which is theoretically limited only by the `usize` max value or memory capacity.

Under the hood [BigDecimal](https://docs.rs/bigdecimal/latest/bigdecimal/struct.BigDecimal.html) uses
a [BigInt](https://docs.rs/num-bigint/latest/num_bigint/struct.BigInt.html) object, paired with a 64-bit integer which
determines the position of the decimal point. Therefore, the precision is not actually arbitrary, but limited to 2<sup>
63</sup> decimal places.

Despite the seemingly undeniable advantages at first glance, this approach also has a number of fundamental
disadvantages:

- Non-copyable types for both integers and fixed point numbers.
- Dynamic allocation to store even very small numbers, for example, `0` or `1`.
- Extra dynamic allocation for almost any operation (mathematical operations, parsing, converting, etc.).
- Constant calculations are not available.
- Potentially uncontrolled growth of memory consumption and the need to artificially limit it.

Because most practical problems requiring the use of fixed-point numbers do not require so much
limit on the number of digits, such as `usize`, but as a rule it is limited:

| Unit                              | Precision        | Decimal digits |
|-----------------------------------|------------------|----------------|
| United States Dollar (USD)        | 0.01             | 2              |
| United States Dollar, stock (USD) | 0.0001           | 4              |
| Bitcoin (BTC)                     | 10<sup>-8</sup>  | 8              |
| Ethereum (ETH)                    | 10<sup>-18</sup> | 18             |

Then most real numbers for financial and other systems requiring accuracy can use 256-bit or even 128-bit
integer to store decimal digits.

So In this library, a different approach was chosen.

### Big Integers

For big integers this crate provides integer types of arbitrary fixed size which behave exactly like Rust's internal
primitive integer types (`u8`, `i8`, `u16`, `i16`, etc.):

| Unit    | Bits | Representation | Signed | Min               | Max                | Helper Macro   |
|---------|------|----------------|--------|-------------------|--------------------|----------------|
| `I128`  | 128  | `2 x u64`      | +      | -2<sup>127</sup>  | 2<sup>127</sup>-1  | `int128!(1)`   |
| `U128`  | 128  | `2 x u64`      |        | 0                 | 2<sup>128</sup>    | `uint128!(1)`  |
| `I256`  | 256  | `4 x u64`      | +      | -2<sup>255</sup>  | 2<sup>255</sup>-1  | `int256!(1)`   |
| `U256`  | 256  | `4 x u64`      |        | 0                 | 2<sup>256</sup>    | `uint256!(1)`  |
| `I512`  | 512  | `8 x u64`      | +      | -2<sup>511</sup>  | 2<sup>511</sup>-1  | `int512!(1)`   |
| `U512`  | 512  | `8 x u64`      |        | 0                 | 2<sup>512</sup>    | `uint512!(1)`  |
| `I1024` | 1024 | `16 x u64`     | +      | -2<sup>1023</sup> | 2<sup>1023</sup>-1 | `int1024!(1)`  |
| `U1024` | 1024 | `16 x u64`     |        | 0                 | 2<sup>1024</sup>   | `uint1024!(1)` |
| `I2048` | 2048 | `32 x u64`     | +      | -2<sup>2047</sup> | 2<sup>2047</sup>-1 | `int2048!(1)`  |
| `U2048` | 2048 | `32 x u64`     |        | 0                 | 2<sup>2048</sup>   | `uint2048!(1)` |
| `I4096` | 4096 | `64 x u64`     | +      | -2<sup>4095</sup> | 2<sup>4095</sup>-1 | `int4096!(1)`  |
| `U4096` | 4096 | `64 x u64`     |        | 0                 | 2<sup>4096</sup>   | `uint4096!(1)` |
| `I8192` | 8192 | `128 x u64`    | +      | -2<sup>8191</sup> | 2<sup>8191</sup>-1 | `int8192!(1)`  |
| `U8192` | 8192 | `128 x u64`    |        | 0                 | 2<sup>8192</sup>   | `uint8192!(1)` |

Nearly all methods defined on Rust's signed and unsigned primitive integers are defined `fastnum`'s signed and unsigned
integers.

Under the hood [bnum](https://docs.rs/bnum/latest/bnum/) is currently used as the backend as most meeting the
requirements.
Subsequently, the implementation can be replaced in favor of its own implementation, which enables `SIMD`.

Unsigned integers are stored as an array of digits (primitive unsigned integers) of length `N`. This means all
`fastnum` integers can be stored on the stack, as they are fixed size. Signed integers are simply stored as an unsigned
integer in two's complement.

### Decimals

`fastnum` provides a several decimal numbers suitable for financial calculations that require significant
integral and fractional digits with no round-off errors.

| Decimal type | Integer part | Bits | Memory representation | Signed | Max significant_digits | Helper macro    |
|--------------|--------------|------|-----------------------|--------|------------------------|-----------------|
| `D128`       | `U128`       | 128  | `2 x u64 + i64 + i64` | +      | 2<sup>128</sup>        | `dec128!(0.1)`  |
| `UD128`      | `U128`       | 128  | `2 x u64 + i64`       |        | 2<sup>128</sup>        | `udec128!(0.1)` |
| `D256`       | `U256`       | 256  | `4 x u64 + i64 + i64` | +      | 2<sup>256</sup>        | `dec256!(0.1)`  |
| `UD256`      | `U256`       | 256  | `4 x u64 + i64`       |        | 2<sup>256</sup>        | `udec256!(0.1)` |
| `D512`       | `U512`       | 512  | `8 x u64 + i64 + i64` | +      | 2<sup>512</sup>        | `dec512!(0.1)`  |
| `UD512`      | `U512`       | 512  | `8 x u64 + i64`       |        | 2<sup>512</sup>        | `udec512!(0.1)` |

Under the hood any `[D|UD]N` decimal type consists of a N-bit big unsigned integer, paired with a 64-bit signed integer
scaling factor which determines the position of the decimal point and sign (for signed types only). Therefore, the
precision is not actually arbitrary, but limited to 2<sup>63</sup> decimal places. Because of this representation,
trailing zeros are preserved and may be exposed when in string form. These can be truncated using the normalize or
round_dp functions.

Thus, fixed-point numbers are trivially copyable and do not require any dynamic allocation. This allows you to get
additional performance gains by eliminating not only dynamic allocation, like such, but also will get rid of one
indirect addressing, which improves cache-friendliness and reduces the CPU load.

## Why fastnum?

- **Trivially copyable types**: all `fastnum` numerics are trivially copyable (both integer and decimal, ether signed
  and unsigned) and can be stored on the stack, as they are fixed size.
- **No dynamic allocation**: no expensive sys-call's, no indirect addressing, cache-friendly.
- **Compile-time integer and decimal parsing**: all the `from_*` methods on `fastnum` integers
  and decimals are `const`, which allows parsing of integers and numerics from string slices and floats at compile time.
  Additionally, the string to be parsed does not have to be a literal: it could, for example, be obtained via [
  `include_str!`](https://doc.rust-lang.org/core/macro.include_str.html), or [
  `env!`](https://doc.rust-lang.org/core/macro.env.html).
- **Const-evaluated in compile time macro-helpers**: any type has its own macro helper which can be used for
  definitions of constants or variables whose value is known in advance. This allows you to perform all the necessary
  checks at the compile time.
- **Small dependencies by default**: `fastnum` does not depend on any other crates by default. Support for crates such
  as [`rand`](https://docs.rs/rand/latest/rand/) and [`serde`](https://docs.rs/serde/latest/serde/) can be enabled with
  crate [features](#features).
- **`no-std` compatible**: `fastnum` can be used in `no_std` environments.
- **`const` evaluation**: nearly all methods defined on `fastnum` integers and decimals are `const`, which allows
  complex compile-time calculations and checks.

## Installation

To install and use `fastnum`, simply add the following line to your `Cargo.toml` file in the `[dependencies]` section:

```toml
fastnum = "0.0.1"
```

Or, to enable various `fastnum` features as well, add for example this line instead:

```toml
fastnum = { version = "0.0.1", features = ["serde"] } # enables the "serde" feature
```

## Example Usage

```rust
use fastnum::{udec256, UD256};

fn main() {
    const ZERO: UD256 = udec256!(0);
    const ONE: UD256 = udec256!(1.0);

    let a = udec256!(12345);

    println!("a = {a}");
}
```

## Features

### Serialization and Deserialization

The `serde` feature enables serialization and deserialization of `fastnum` decimals via the [
`serde`](https://docs.rs/serde/latest/serde/) crate. More details about serialization and deserialization you can found
in

### Database ORM's support

The `diesel` feature enables serialization and deserialization of `fastnum` decimals for [
`diesel`](https://docs.rs/diesel/latest/diesel/) crate.

The `sqlx` feature enables serialization and deserialization of `fastnum` decimals for [
`sqlx`](https://docs.rs/sqlx/latest/sqlx/) crate.

### Autodocs crates support

The `utoipa` feature enables support of `fastnum` decimals for autogenerated OpenAPI documentation via the [
`utoipa`](https://docs.rs/utoipa/latest/utoipa/) crate.

## Testing

This crate is tested with the [`rstest`](https://docs.rs/rstest/latest/rstest/) crate as well as with specific edge
cases.

## Minimum Supported Rust Version

The current Minimum Supported Rust Version (MSRV) is `1.82.0`.

## Documentation

If a method is not documented explicitly, it will have a link to the equivalent method defined on primitive Rust
integers (since the methods have the same functionality).

**NB: `fastnum` is currently pre-`1.0.0`. As per the [Semantic Versioning guidelines](https://semver.org/#spec-item-4), the
public API may contain breaking changes while it is in this stage. However, as the API is designed to be as similar as
possible to the API of Rust's primitive integers, it is unlikely that there will be a large number of breaking changes.
**

## Compile-Time Configuration

You can set a few default parameters at _compile-time_ via environment variables:

| Environment Variable                           | Default    |
|------------------------------------------------|------------|
| `RUST_FASTNUM_DEFAULT_PRECISION`               | 100        |
| `RUST_FASTNUM_DEFAULT_ROUNDING_MODE`           | `HalfEven` |
| `RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD` | 5          |
| `RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD` | 15         |
| `RUST_FASTNUM_FMT_MAX_INTEGER_PADDING`         | 1000       |
| `RUST_FASTNUM_DEFAULT_SERDE_DESERIALIZE_MODE`  | `Strict`   |

Examine [build.rs] for how those are converted to constants in the code (if interested).

[build.rs]: ./build.rs

## Future Work

There are several areas for further work:

- Micro-optimization of big integer types using vector extensions (SSE2, SSE4.2, AVX2, AVX512F, etc.).
- Const trait implementations once they are stabilized in Rust. (https://github.com/rust-lang/rust/issues/67792)
- Integration with a large number of crates (ORM's, auto-docs crates, etc.).

## Licensing

This code is dual-licensed under the permissive
[MIT](https://opensource.org/licenses/MIT) &
[Apache 2.0](https://opensource.org/licenses/Apache-2.0) licenses.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
