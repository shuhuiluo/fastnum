# fastnum

Fixed-size signed and unsigned decimal numbers implemented in pure Rust. Suitable for
financial, crypto and any other fixed-precision calculations.

## Overview

`fastnum` provides signed and unsigned fixed precision decimal numbers suitable for financial calculations that
require significant integral and fractional digits with no round-off errors (such as 0.1 + 0.2 ≠ 0.3).

Under the hood any decimal type consists of a N-bit big unsigned integer, paired with a 64-bit signed integer
scaling factor which determines the position of the decimal point and sign (for signed types only). Trailing zeros are
preserved and may be exposed when in string form. These can be truncated using the normalize or
round functions.

Thus, fixed-point numbers are trivially copyable and do not require any dynamic allocation. This allows you to get
additional performance gains by eliminating not only dynamic allocation, like such, but also will get rid of one
indirect addressing, which improves cache-friendliness and reduces the CPU load.

### Why fastnum?

- **Blazing fast**: `fastnum` numerics are as fast as native types, well almost :).
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
- **Short dependencies list by default**: `fastnum` does not depend on many other crates by default. Support for crates
  such
  as [`rand`](https://docs.rs/rand/latest/rand/) and [`serde`](https://docs.rs/serde/latest/serde/) can be enabled with
  crate [features](#features).
- **`no-std` compatible**: `fastnum` can be used in `no_std` environments.
- **`const` evaluation**: nearly all methods defined on `fastnum` integers and decimals are `const`, which allows
  complex compile-time calculations and checks.

## Installation

To install and use `fastnum`, simply add the following line to your `Cargo.toml` file in the `[dependencies]` section:

```toml
fastnum = "0.0.12"
```

Or, to enable various `fastnum` features as well, add for example this line instead:

```toml
fastnum = { version = "0.0.12", features = ["serde"] } # enables the "serde" feature
```

## Example Usage

```
use fastnum::{udec256, UD256};

let a = udec256!(0.1);
let b = udec256!(0.2);

assert_eq!(a + b, udec256!(0.3));
```

## Const-evaluated in compile time macro-helpers

Any type has its own macro helper which can be used for definitions of constants or variables whose value is known in
advance. This allows you to perform all the necessary checks at the compile time.

| Decimal type | Integer part | Bits | Signed | Helper macro    |
|--------------|--------------|------|:------:|-----------------|
| `D128`       | `U128`       | 128  |   ✅    | `dec128!(0.1)`  |
| `UD128`      | `U128`       | 128  |        | `udec128!(0.1)` |
| `D256`       | `U256`       | 256  |   ✅    | `dec256!(0.1)`  |
| `UD256`      | `U256`       | 256  |        | `udec256!(0.1)` |
| `D512`       | `U512`       | 512  |   ✅    | `dec512!(0.1)`  |
| `UD512`      | `U512`       | 512  |        | `udec512!(0.1)` |
| `D128`       | `U128`       | 128  |   ✅    | `dec128!(0.1)`  |
| `UD128`      | `U128`       | 128  |        | `udec128!(0.1)` |
| `D256`       | `U256`       | 256  |   ✅    | `dec256!(0.1)`  |
| `UD256`      | `U256`       | 256  |        | `udec256!(0.1)` |
| `D512`       | `U512`       | 512  |   ✅    | `dec512!(0.1)`  |
| `UD512`      | `U512`       | 512  |        | `udec512!(0.1)` |

## Arithmetic

### Arithmetic operations

All arithmetic operations over decimals are fixed precision.

### Arithmetic result

[ArithmeticResult]: #arithmetic-result

The result of any arithmetic operation over decimal `T` type is [`DecimalResult<T>`](crate::decimal::DecimalResult) -
special type wrapper extends the original decimal type with set of emergency bit flags:

| **Flag**         | Description                                                                            |
|------------------|----------------------------------------------------------------------------------------|
| `OVERFLOW`       | Indicates that the decimal result of an operation is too large to fit the target type. |
| `INEXACT`        | Rounding was performed during the operation. The result may not be exact.              |
| `DIVIDE_BY_ZERO` | Division by zero.                                                                      |
| `NEGATIVE`       | The negative result cannot be represented by an unsigned type.                         |

Emergency flags are very similar to the [processor flag register](https://en.wikipedia.org/wiki/FLAGS_register) and
contain information about arithmetic errors such as operation overflow, division by 0, or loss of precision
calculations.

#### Overflow

`OVERFLOW` happens when the result of an arithmetic operation is too large to be stored in the designated destination
area.

This is strong version of `INEXACT` flag when no rounding allows reducing the dimension of a number and it
does not fit into target type. For example, `UD128::MAX * UD128::MAX` cannot be stored in `UD128` even with rounding
applied and loss of precision.

#### Inexact

`INEXACT` flag indicates that rounding was performed during the operation execution and the result may not be exact.
This is weak version of `OVERFLOW` flag: overflow caused by the operation can be "compensated" by rounding.

For example, `1/3 = 0.333333333333(3)`. The result of division is an infinite decimal fraction which cannot
be stored in any of the existing types: performing the operation will cause `OVERFLOW` for any finite count of
decimal digits However, the result can be obtained if we make a deal and agree to the loss of precision.
In this case, the result will be the rounded value, accompanied by information that rounding has been performed and
the result may not be accurate.

|         | Result            | Rounded (HalfUp) | Rounded (Down) | `INEXACT` |
|---------|-------------------|------------------|----------------|:---------:|
| `1 + 1` | 2                 | 2                | 2              |           |
| `1 / 3` | 0.333333333333(3) | 0.333333333333   | 0.333333333333 |    ⁉️     |
| `2 / 3` | 0.666666666666(6) | 0.666666666667   | 0.666666666666 |    ⁉️     |

For most practical purposes this is an acceptable trade-off. However, we always leave the possibility
be sure that the result is accurate and there were no loss of precision.

#### Divide by zero

`DIVIDE_BY_ZERO` flag indicates that an attempt to divide by `0` was made.

#### Negative

`NEGATIVE` flag indicates that the result of a mathematical operation over unsigned decimal is negative and cannot be
represented by current unsigned type.

[Arithmetic result](crate::decimal::DecimalResult) can be unwrapped into target type `T` with specific or
default [ArithmeticPolicy] (see methods [crate::decimal::DecimalResult::unwrap] and
[crate::decimal::DecimalResult::unwrap_with_policy]).

```
use fastnum::udec128;
use fastnum::decimal::RoundingMode;

let a = udec128!(1);
let b = udec128!(3);

let c = a.div(b, RoundingMode::default()).unwrap();

assert_eq!(c, udec128!(0.333333333333333333333333333333333333333));
```

```should_panic
use fastnum::udec256;
use fastnum::decimal::RoundingMode;

let a = udec256!(1);
let b = udec256!(0);

let c = a.div(b, RoundingMode::default()).unwrap();
```

Or can be converted into [Option] or [Result].

```
use fastnum::udec128;
use fastnum::decimal::RoundingMode;

let a = udec128!(1);
let b = udec128!(3);

let res = a.div(b, RoundingMode::default()).ok();

assert!(res.is_none());
```

```
use fastnum::udec128;
use fastnum::decimal::{RoundingMode, ArithmeticError};

let a = udec128!(1);
let b = udec128!(3);

let res = a.div(b, RoundingMode::default()).ok_or_err();

assert_eq!(res, Err(ArithmeticError::Inexact));
```

### Arithmetic policy

[ArithmeticPolicy]: #arithmetic-policy

Arithmetic policy defines the rules for unwrapping the result of an arithmetic operation containing emergency
flags to get the result:

```
use fastnum::{udec256, UD256};
use fastnum::decimal::{ArithmeticPolicy, OverflowPolicy, RoundingMode, RoundingPolicy};

let a = UD256::ONE;
let b = UD256::TWO;

let policy = ArithmeticPolicy::new(OverflowPolicy::Strict, RoundingPolicy::Strict);

let c = a.add(b, RoundingMode::default()).unwrap_with_policy(&policy);
assert_eq!(c, udec256!(3));
```

Saturate if overflowed:

```
use fastnum::{udec256, UD256};
use fastnum::decimal::{ArithmeticPolicy, OverflowPolicy, RoundingMode, RoundingPolicy};

let a = UD256::MAX;
let b = UD256::MAX;

let policy = ArithmeticPolicy::new(OverflowPolicy::Saturating, RoundingPolicy::Strict);

let c = a.add(b, RoundingMode::default()).unwrap_with_policy(&policy);
assert_eq!(c, udec256!(115792089237316195423570985008687907853269984665640564039457584007913129639934e9223372036854775808));
```

Should panic:

```should_panic
use fastnum::{udec256, UD256};
use fastnum::decimal::{ArithmeticPolicy, OverflowPolicy, RoundingMode, RoundingPolicy};

let a = udec256!(1);
let b = udec256!(3);

let policy = ArithmeticPolicy::new(OverflowPolicy::Saturating, RoundingPolicy::Strict);

let c = a.div(b, RoundingMode::default()).unwrap_with_policy(&policy);
```

#### Overflow policy

`Overflow policy` defines the rules for unwrapping the result of an arithmetic operation containing overflow emergency
flag.

| **Policy**   | Description                            | Default |
|--------------|----------------------------------------|:-------:|
| `Strict`     | Panic if overflow occurred.            |    ✅    |
| `Saturating` | Saturating value if overflow occurred. |         |

#### Rounding policy

| **Policy** | Description                            | Default |
|------------|----------------------------------------|:-------:|
| `Strict`   | Panic if overflow occurred.            |         |
| `Round`    | Saturating value if overflow occurred. |    ✅    |

#### Default Arithmetic policy

[DefaultArithmeticPolicy]: #default-arithmetic-policy

The behaviour of the implementation of this method is the same as for Rust's primitive integers - i.e.
in debug mode it panics on overflow, and in release mode it performs

| **Policy**      | Debug mode | Release mode |
|-----------------|------------|--------------|
| Overflow policy | `Strict`   | `Saturating` |
| Rounding policy | `Round`    | `Round`      |

Summary unwrapping Decimal Result with flags has behavior:

| **Flag**         | Debug mode | Release mode |
|------------------|------------|--------------|
| `OVERFLOW`       | ❗panic     | saturate     |
| `INEXACT`        | saturate   | saturate     |
| `DIVIDE_BY_ZERO` | ❗panic     | ❗panic       |
| `NEGATIVE`       | ❗panic     | ❗panic       |

### Rust operators overloads

[RustOperatorsOverloads]: #rust-operators-overloads

Common numerical operations (such as addition operator `+`, addition assignment operator `+=`, division operator `/`,
etc...) are overloaded for `fastnum` decimals, so we can treat them the same way we treat other numbers.

```
use fastnum::udec256;

let a = udec256!(3.5);
let b = udec256!(2.5);
let c = a + b;

assert_eq!(c, udec256!(6));
```

Unfortunately current version of Rust does not support const traits, so this example fail to compile:

```compile_fail
use fastnum::{udec256, UD256};

const A: UD256 = udec256!(3.5);
const B: UD256 = udec256!(2.5);
const C: UD256 = A + B;
```

In constant calculations and static contexts, until the [`feature`](https://github.com/rust-lang/rust/issues/67792) is
stabilized, the following const methods should be used:

```
use fastnum::{udec256, UD256};
use fastnum::decimal::RoundingMode;

const A: UD256 = udec256!(3.5);
const B: UD256 = udec256!(2.5);
const C: UD256 = A.add(B, RoundingMode::default()).unwrap();

assert_eq!(C, udec256!(6));
```

**Note**: All Rust overloaded operators uses [`unwrap`](decimal::DecimalResult::unwrap) method of DecimalResult so
the [DefaultArithmeticPolicy] is used.
It that prescion loss overflow is ...

## Sign

## Rounding

### Rounding mode

[RoundingMode]: #rounding-mode

[RoundingMode](crate::decimal::RoundingMode) enum determines how to calculate the last digit of the number when
performing
rounding:

| Mode       | Description                                                                   | Default | Examples                                                                                                                     |
|------------|-------------------------------------------------------------------------------|:-------:|------------------------------------------------------------------------------------------------------------------------------|
| `Up`       | Always round away from zero                                                   |         | * 5.5 → 6.0<br>* 2.5 → 3.0<br>* 1.6 → 2.0<br>* 1.1 → 2.0<br>* -1.1 → -2.0<br>* -1.6 → -2.0<br>* -2.5 → -3.0<br>* -5.5 → -6.0 |
| `Down`     | Always round towards zero                                                     |         | * 5.5 → 5.0<br>* 2.5 → 2.0<br>* 1.6 → 1.0<br>* 1.1 → 1.0<br>* -1.1 → -1.0<br>* -1.6 → -1.0<br>* -2.5 → -2.0<br>* -5.5 → -5.0 |
| `Ceiling`  | Round towards +∞                                                              |         | * 5.5 → 6.0<br>* 2.5 → 3.0<br>* 1.6 → 2.0<br>* 1.1 → 2.0<br>* -1.1 → -1.0<br>* -1.6 → -1.0<br>* -2.5 → -2.0<br>* -5.5 → -5.0 |
| `Floor`    | Round towards -∞                                                              |         | * 5.5 → 5.0<br>* 2.5 → 2.0<br>* 1.6 → 1.0<br>* 1.1 → 1.0<br>* -1.1 → -2.0<br>* -1.6 → -2.0<br>* -2.5 → -3.0<br>* -5.5 → -6.0 |
| `HalfUp`   | Round to 'nearest neighbor', or up if ending decimal is 5                     |    ✅    | * 5.5 → 6.0<br>* 2.5 → 3.0<br>* 1.6 → 2.0<br>* 1.1 → 1.0<br>* -1.1 → -1.0<br>* -1.6 → -2.0<br>* -2.5 → -3.0<br>* -5.5 → -6.0 |
| `HalfDown` | Round to 'nearest neighbor', or down if ending decimal is 5                   |         | * 5.5 → 5.0<br>* 2.5 → 2.0<br>* 1.6 → 2.0<br>* 1.1 → 1.0<br>* -1.1 → -1.0<br>* -1.6 → -2.0<br>* -2.5 → -2.0<br>* -5.5 → -5.0 |
| `HalfEven` | Round to 'nearest neighbor', if equidistant, round towards nearest even digit |         | * 5.5 → 6.0<br>* 2.5 → 2.0<br>* 1.6 → 2.0<br>* 1.1 → 1.0<br>* -1.1 → -1.0<br>* -1.6 → -2.0<br>* -2.5 → -2.0<br>* -5.5 → -6.0 |

## Formatting

Rust's `fmt::Display` formatting options for `fastnum` decimals:

| Placeholder     | Format                                         | Description                                                                                             |
|-----------------|------------------------------------------------|---------------------------------------------------------------------------------------------------------|
| `{}`            | Default Display                                | Format as "human readable" number.                                                                      |
| `{:.<PREC>}`    | Display with precision                         | Format number with exactly `PREC` digits after the decimal place.                                       |
| `{:e}` / `{:E}` | Exponential format                             | Formats in scientific notation with either `e` or `E` as exponent delimiter. Precision is kept exactly. |
| `{:.<PREC>e}`   | Formats in scientific notation, keeping number | Number is rounded / zero padded until.                                                                  |
| `{:?}`          | Debug                                          | Shows internal representation of decimal.                                                               |
| `{:#?}`         | Alternate Debug (used by `dbg!()`)             | Shows simple int+exponent string representation of decimal.                                             |

### Default Display

- "Small" fractional numbers (close to zero) are printed in scientific notation
    - Number is considered "small" by number of leading zeros exceeding a threshold
    - Configurable by the compile-time environment variable:
      [RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD](#compile-time-configuration)
        - Default `5`
    - Example: `1.23e-3` will print as `0.00123` but `1.23e-10` will be `1.23E-10`
- Trailing zeros will be added to "small" integers, avoiding scientific notation
    - May appear to have more precision than they do
    - Example: decimal `1e1` would be rendered as `10`
    - The threshold for "small" is configured by compile-time environment variable:
      [`RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD`](#compile-time-configuration)
        - Default `15`
    - `1e15` => `1000000000000000`
    - Large integers (e.g. `1e50000000`) will print in scientific notation, not
      a 1 followed by fifty million zeros
- All other numbers are printed in standard decimal notation

### Display with precision

Numbers with fractional components will be rounded at precision point, or have zeros padded to precision point. Integers
will have zeros padded to the precision point. To prevent unreasonably sized output, a threshold limits the number of
padded zeros:

- Greater than the default case, since specific precision was requested
- Configurable by the compile-time environment variable:
  [`RUST_BIGDECIMAL_FMT_MAX_INTEGER_PADDING`](#compile-time-configuration)
- Default `1000`

If digits exceed this threshold, they are printed without decimal-point, suffixed with scale of the decimal.

## Serialization

[Serialization]: #serialization

If you are passing decimal numbers between systems, be sure to use a serialization format
which explicitly supports decimal numbers and does not require transformations to
floating-point binary numbers, or there will be information loss.

Text formats like JSON should work ok as long as the receiver will also parse
numbers as decimals so complete precision is kept accurate.
Typically, JSON-parsing implementations do not do this by default, and need special
configuration.

Binary formats like msgpack may expect/require representing numbers as 64-bit IEEE-754
floating-point, and will likely lose precision by default unless you explicitly format
the decimal as a string, bytes, or some custom structure.

### Serde

`serde` feature allows to serialize and deserialize `fasnum` decimals via the [
`serde`](https://docs.rs/serde/latest/serde/) crate.

- Decimals are always serialized as _as a string_.
- Decimals deserialize behavior defined via [`RUST_FASTNUM_SERDE_DESERIALIZE_MODE`](#compile-time-configuration).
  Possible values are:

| **Mode**    | Description                                                                                                                                                                | Default |
|-------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:-------:|
| `Strict`    | Allow only string values such as `"0.1"`, `"0.25"`, etc. Any other numbers like `0.1` or `1` triggers parsing error.                                                       |    ✅    |
| `Stringify` | Decimal values such as `0.1` will be stringify to `"0.1"` first and then parse as decimal numbers.                                                                         |         |
| `Any`       | Allow any values. Parse performs in-place. Result may be inexact because there is no correct limited binary representation for some floating-point numbers, such as `0.1`. |         |

```toml
[dependencies]
fastnum = { version = "0.0.6", features = ["serde"] } 
```

Basic usage:

```
use fastnum::{udec256, UD256};
use serde::*;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    name: String,
    value: UD256,
}

let json_src = r#"{ "name": "foo", "value": "1234567e-3" }"#;

let my_struct: MyStruct = serde_json::from_str(&json_src).unwrap();
dbg!(&my_struct);
// MyStruct { name: "foo", value: UD256("1234.567") }

println!("{}", serde_json::to_string(&my_struct).unwrap());
// {"name":"foo","value":"1234.567"}
```

Should panic:

```should_panic
use fastnum::{udec256, UD256};
use serde::*;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    name: String,
    value: UD256,
}

let json_src = r#"{ "name": "foo", "value": 1234567e-3 }"#;
let my_struct: MyStruct = serde_json::from_str(&json_src).unwrap();
```

## Features

| Feature           | Default | Description                                                                                                                                                                                                                                                                                                                                                                                  |
|-------------------|:-------:|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `std`             |    ✅    |                                                                                                                                                                                                                                                                                                                                                                                              |
| `numtraits`       |         | Includes implementations of traits from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/) and [`num_integer`](https://docs.rs/num-integer/latest/num_integer/) crates, e.g. [`AsPrimitive`](https://docs.rs/num-traits/latest/num_traits/cast/trait.AsPrimitive.html), [`Signed`](https://docs.rs/num-traits/latest/num_traits/sign/trait.Stper/trait.Integer.html) and etc. |
| `rand`            |         | Allows creation of random `fastnum` decimals via the [`rand`](https://docs.rs/rand/latest/rand/) crate.                                                                                                                                                                                                                                                                                      |
| `zeroize`         |         | Enables the [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) trait implementation from the [`zeroize`](https://docs.rs/zeroize/latest/zeroize/) crate for `fastnum` decimals.                                                                                                                                                                                          |
| `serde`           |         | Enables serialization and deserialization of `fastnum` decimals via the [`serde`](https://docs.rs/serde/latest/serde/) crate.                                                                                                                                                                                                                                                                |
| `diesel`          |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) crate.                                                                                                                                                                                                                                                                 |
| `diesel_postgres` |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) PostgreSQL backend.                                                                                                                                                                                                                                                    |
| `diesel_mysql`    |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) MySQL backend.                                                                                                                                                                                                                                                         |
| `sqlx`            |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) crate.                                                                                                                                                                                                                                                                       |
| `sqlx_postgres`   |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) PostgreSQL backend.                                                                                                                                                                                                                                                          |
| `sqlx_mysql`      |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) MySQL backend.                                                                                                                                                                                                                                                               |
| `utoipa`          |         | Enables support of `fastnum` decimals for autogenerated OpenAPI documentation via the [`utoipa`](https://docs.rs/utoipa/latest/utoipa/) crate.                                                                                                                                                                                                                                               |

### Compile-Time Configuration

You can set a few default parameters at _compile-time_ via environment variables:

| Environment Variable                           | Default  |
|------------------------------------------------|----------|
| `RUST_FASTNUM_DEFAULT_ROUNDING_MODE`           | `HalfUp` |
| `RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD` | `5`      |
| `RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD` | `15`     |
| `RUST_FASTNUM_FMT_MAX_INTEGER_PADDING`         | `1000`   |
| `RUST_FASTNUM_SERDE_DESERIALIZE_MODE`          | `Strict` |
