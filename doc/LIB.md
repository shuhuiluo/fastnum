# fastnum

Fixed-size signed and unsigned decimal numbers implemented in pure Rust. Suitable for
financial, crypto and any other fixed-precision calculations.

## Overview

`fastnum` provides signed and unsigned exact precision decimal numbers suitable for financial calculations that
require significant integral and fractional digits with no round-off errors (such as 0.1 + 0.2 ≠ 0.3).

Under the hood any decimal type consists of a N-bit big unsigned integer, paired with a 64-bit signed integer
scaling factor which determines the position of the decimal point and sign (for signed types only). Trailing zeros are
preserved and may be exposed when in string form. These can be truncated using the normalize or
round functions.

Thus, `fastnum` decimal numbers are trivially copyable and do not require any dynamic allocation. This allows you to get
additional performance gains by eliminating not only dynamic allocation, like such, but also will get rid of one
indirect addressing, which improves cache-friendliness and reduces the CPU load.

### Why fastnum?

- **Strictly exact precision**: no round-off errors (such as 0.1 + 0.2 ≠ 0.3).
- **Blazing fast**: `fastnum` numerics are as fast as native types, well almost :).
- **Trivially copyable types**: all `fastnum` numerics are trivially copyable (both integer and decimal, ether signed
  and unsigned) and can be stored on the stack, as they are fixed size.
- **No dynamic allocation**: no expensive sys-call's, no indirect addressing, cache-friendly.
- **Compile-time integer and decimal parsing**: all the `from_*` methods on `fastnum` integers
  and decimals are `const`, which allows parsing of integers and numerics from string slices and floats at compile time.
  Additionally, the string to be parsed does not have to be a literal: it could, for example, be obtained via [
  `include_str!`](https://doc.rust-lang.org/core/macro.include_str.html), or [
  `env!`](https://doc.rust-lang.org/core/macro.env.html).
- **Const-evaluated in compile time macro-helpers**: any type has
  its [own macro helper](#const-evaluated-in-compile-time-macro-helpers) which can be used for
  definitions of constants or variables whose value is known in advance. This allows you to perform all the necessary
  checks at the compile time.
- **Short dependencies list by default**: `fastnum` does not depend on many other crates by default. Support for crates
  such as [`rand`](https://docs.rs/rand/latest/rand/) and [`serde`](https://docs.rs/serde/latest/serde/) can be enabled
  with crate [features](#features).
- **`no-std` compatible**: `fastnum` can be used in `no_std` environments.
- **`const` evaluation**: nearly all methods defined on `fastnum` integers and decimals are `const`, which allows
  complex compile-time calculations and checks.

## Installation

To install and use `fastnum`, simply add the following line to your `Cargo.toml` file in the `[dependencies]` section:

```toml
fastnum = "0.0.14"
```

Or, to enable various `fastnum` features as well, add for example this line instead:

```toml
fastnum = { version = "0.0.14", features = ["serde"] } # enables the "serde" feature
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

### Examples

Basic usage:

```
use fastnum::{udec256, UD256};

// This value will be evaluated at compile-time and inlined directly into the relevant context when used. 
const PI: UD256 = udec256!(3.141592653589793115997963468544185161590576171875);
```

Compile-time calculations:

```
use fastnum::{udec256, UD256, decimal::RoundingMode};

// This value will be evaluated at compile-time and inlined directly into the relevant context when used. 
const PI_X_2: UD256 = udec256!(2).mul(udec256!(3.141592653589793115997963468544185161590576171875), RoundingMode::default()).unwrap();
```

Compile-time checks

```compile_fail
use fastnum::{udec256, UD256};

// Invalid character.
const E: UD256 = udec256!(A3.5);
```

```compile_fail
use fastnum::{udec256, UD256, decimal::RoundingMode};

// Arithmetic error during calculation.
const E: UD256 = udec256!(1.5).div(udec256!(0), RoundingMode::default()).unwrap();
```

## Representation

Under the hood any finite N-bit unsigned decimal type consists of a N-bit big unsigned integer, paired with a signed
integer scaling factor:

coefficient × 10<sup>-exp</sup>, where

- **coefficient** (or _significant integral digits_) – an unsigned N-bit big integer which is zero or positive.
- **scaling factor** (or _exponent_) – a signed 64-bit integer which determines the position of the decimal point and
  indicates the power of ten by which the coefficient is multiplied.

Trailing zeros are preserved and may be exposed when in string form. These can be truncated using the normalize or
round functions. The quantum of a finite number is given by: 1 × 10<sup>-exp</sup>. This is the value of a unit in the
least significant position of the coefficient of a finite number.

Signed N-bit decimal is represented as:

sign × UD\[coefficient × 10<sup>-exp</sup>\], where

- **UD** - N-bit unsigned decimal
- **sign** - a value indicates that the number is negative or positive.

A number with a coefficient of `0` is permitted to have both `+` and `minus` sign. Negative zero is accepted as an
operand for all operations (see [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)).

## Precision

Precision is integral number of decimal digits. It is limited by maximum decimal digits that N-bit unsigned coefficient
can hold.

## Arithmetic

The `fastnum` crate provides support for fast correctly rounded decimal floating-point arithmetic. It offers several
advantages over the native floating-point ([IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)) types:

### Exact precision

`fastnum` decimals provide an arithmetic that works in the same way as the arithmetic that people learn
at school. Decimal numbers can be represented exactly. In contrast, numbers like `1.1` and `2.2` do not have exact
representations in binary floating point. End users typically would not expect `1.1 + 2.2` to display as
`3.3000000000000003` as it does with binary floating point.

Floats:

```
let n: f64 = 1.1 + 2.2;
assert_eq!(n.to_string(), "3.3000000000000003");
```

Decimals:

```
use fastnum::udec256;

let n = udec256!(1.1) + udec256!(2.2);
assert_eq!(n.to_string(), "3.3");
```

The exactness carries over into arithmetic. In decimal floating point, `0.1 + 0.1 + 0.1 - 0.3` is exactly equal to zero.
In binary floating point, the result is `5.5511151231257827e-017`. While near to zero, the differences prevent reliable
equality testing and differences can accumulate. For this reason, decimal is preferred in accounting applications which
have strict equality invariants.

Floats:

```
let n: f64 = 0.1 + 0.1 + 0.1 - 0.3;
assert_eq!(n.to_string(), "0.00000000000000005551115123125783");
```

Decimals:

```
use fastnum::udec256;

let n = udec256!(0.1) + udec256!(0.1) + udec256!(0.1) - udec256!(0.3);
assert_eq!(n.to_string(), "0");
assert!(n.is_zero());
```

### Fixed precision

`fastnum` incorporates a notion of significant places so that `1.30 + 1.20` is `2.50`. The trailing zero is kept
to indicate significance. This is the customary presentation for monetary applications. For multiplication, the
“schoolbook” approach uses all the figures in the multiplicands. For instance, `1.3 * 1.2` gives `1.56` while
`1.30 * 1.20` gives `1.5600`.

Decimals:

```
use fastnum::udec256;

let n = udec256!(1.30) + udec256!(1.20);
assert_eq!(n, udec256!(2.50));

let n = udec256!(1.3) * udec256!(1.2);
assert_eq!(n, udec256!(1.56));

let n = udec256!(1.30) * udec256!(1.20);
assert_eq!(n, udec256!(1.5600));
```

To preserve significance, the significant digits do not truncate trailing zeros. _Decimals also include special values
such as `Infinity`, `-Infinity`, and `NaN`_. The standard also differentiates `-0` from `+0`.

More about decimal
arithmetic: [IBM’s General Decimal Arithmetic Specification](https://speleotrove.com/decimal/decarith.html).

### Arithmetic rules

1. Every operation on finite numbers is carried out as though an exact mathematical result is computed, using integer
   arithmetic on the coefficient (or significant decimal digits) where possible.

2. If the coefficient of the theoretical exact result has no more than precision digits, then (unless there is an
   underflow or overflow) it is used for the result without change. Otherwise (it has more than precision digits) it is
   rounded (shortened) to exactly precision digits, using the current rounding algorithm, and the exponent is increased
   by the number of digits removed.

3. If the value of the adjusted exponent of the result is less than E<sub>min</sub> (that is, the result is zero or
   subnormal), the calculated coefficient and exponent form the result, unless the value of the exponent is less than
   E<sub>tiny</sub>, in which case the exponent will be set to E<sub>tiny</sub>, the coefficient will be rounded (if
   necessary, and possibly to zero) to match the adjustment of the exponent, and the sign is unchanged.

4. If the value of the adjusted exponent of a non-zero result is larger than E<sub>max</sub>, then an exceptional
   condition (overflow) results. In this case, the result is as defined under the Overflow exceptional condition, and
   may be infinite. It will have the same sign as the theoretical result.

5. The Invalid operation condition may be raised when an operand to an operation is invalid (for example, if it
   exceeds the bounds that an implementation can handle, or the operation is a logarithm and the operand is negative).

6. The sign of the result of a multiplication or division will be `-` only if the operands have different signs.

7. Operands may have more than precision digits and are not rounded before use.

8. [Arithmetic policy] defines the rules for unwrapping the result of an arithmetic operation containing
   [emergency flags](#emergency-flags).

9. Trailing zeros are not removed after operations. The reduce operation may be used to remove trailing zeros if
   desired.

### Arithmetic operations

All arithmetic operations over decimals are exact.

| Operation |   Rust operator   |                   Unsigned                    |                                             Signed                                             |
|-----------|:-----------------:|:---------------------------------------------:|:----------------------------------------------------------------------------------------------:|
| abs       |         ➖         |                       ➖                       | [`abs`](crate::decimal::Decimal::abs), [`unsigned_abs`](crate::decimal::Decimal::unsigned_abs) |
| add       | `a + b`, `a += b` | [`add`](crate::decimal::UnsignedDecimal::add) |                             [`add`](crate::decimal::Decimal::add)                              |
| subtract  | `a - b`, `a -= b` | [`sub`](crate::decimal::UnsignedDecimal::sub) |                             [`sub`](crate::decimal::Decimal::sub)                              |
| multiply  | `a * b`, `a *= b` | [`mul`](crate::decimal::UnsignedDecimal::mul) |                             [`mul`](crate::decimal::Decimal::mul)                              |
| divide    | `a / b`, `a /= b` | [`div`](crate::decimal::UnsignedDecimal::div) |                             [`div`](crate::decimal::Decimal::div)                              |
| remainder | `a % b`, `a %= b` | [`rem`](crate::decimal::UnsignedDecimal::rem) |                             [`rem`](crate::decimal::Decimal::rem)                              |
| negation  |       `-a`        | [`neg`](crate::decimal::UnsignedDecimal::neg) |                             [`neg`](crate::decimal::Decimal::neg)                              |

#### Abs

The _absolute value_ or _modulus_ of a decimal number, denoted, is the non-negative value of without regard to its sign.

The [`unsigned_abs`](crate::decimal::Decimal::unsigned_abs) method returns [
`UnsignedDecimal`](crate::decimal::UnsignedDecimal).

##### Examples:

```
use fastnum::{udec256, dec256};

assert_eq!(dec256!(1.3).abs(), dec256!(1.3));
assert_eq!(dec256!(-1.3).abs(), dec256!(1.3));
assert_eq!(dec256!(-1.3).unsigned_abs(), udec256!(1.3));
```

#### Addition and subtraction

Add and subtract both take two operands. The operands are added (after inverting the sign used for the second operand if
the operation is a subtraction), as follows:

1. The coefficient of the result is computed by adding or subtracting the aligned coefficients of the two operands. The
   aligned coefficients are computed by comparing the exponents of the operands:
    1. If they have the same exponent, the aligned coefficients are the same as the original coefficients.
    2. Otherwise, the aligned coefficient of the number with the larger exponent is its original coefficient multiplied
       by 10n, where n is the absolute difference between the exponents, and the aligned coefficient of the other
       operand is the same as its original coefficient.
    3. If the signs of the operands differ then the smaller aligned coefficient is subtracted from the larger; otherwise
       they are added.
2. The exponent of the result is the minimum of the exponents of the two operands.
3. The sign of the result is determined as follows:
    1. If the result is non-zero then the sign of the result is the sign of the operand having the larger absolute
       value.
    2. Otherwise, the sign of a zero result is 0 unless either both operands were negative or the signs of the operands
       were different and the rounding is round-floor.

##### Examples:

```
use fastnum::{udec256, dec256};

assert_eq!(udec256!(12) + udec256!(7.00), udec256!(19.00));
assert_eq!(udec256!(1E+2) + udec256!(1E+4), udec256!(1.01E+4));
assert_eq!(udec256!(1.3) - udec256!(1.07), udec256!(0.23));
assert_eq!(udec256!(1.3) - udec256!(1.30), udec256!(0.00));
assert_eq!(dec256!(1.3) - dec256!(2.07), dec256!(-0.77));
```

### Arithmetic result

[ArithmeticResult]: #arithmetic-result

The result of any arithmetic operation over decimal `T` type is [`DecimalResult<T>`](crate::decimal::DecimalResult) -
special type wrapper extends the original decimal type with set of emergency bit flags:

#### Emergency flags

| **Flag**         | Description                                                                            |
|------------------|----------------------------------------------------------------------------------------|
| `OVERFLOW`       | Indicates that the decimal result of an operation is too large to fit the target type. |
| `INEXACT`        | Rounding was performed during the operation. The result may not be exact.              |
| `DIVIDE_BY_ZERO` | Division by zero.                                                                      |
| `NEGATIVE`       | The negative result cannot be represented by an unsigned type.                         |
| `INVALID`        | Invalid operation.                                                                     |

Emergency flags are very similar to the [processor flag register](https://en.wikipedia.org/wiki/FLAGS_register) and
contain information about arithmetic errors such as operation overflow, division by `0`, or loss of precision
calculations. Depending on the needs of the application, flags may be ignored, considered as informational, or cause
`panic!`.

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

#### Invalid operation

`INVALID` flag indicates may be raised when an operand to an operation is invalid (for example, if it
exceeds the bounds that an implementation can handle, or the operation is a logarithm and the operand is negative).

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

let res = a.div(b, RoundingMode::default());

assert!(res.is_inexact());
assert_eq!(res.ok(), Some(udec128!(0.333333333333333333333333333333333333333)));
```

```
use fastnum::udec128;
use fastnum::decimal::{RoundingMode, ArithmeticError};

let a = udec128!(1);
let b = udec128!(3);

let res = a.div(b, RoundingMode::default());

assert!(res.is_inexact());
assert_eq!(res.ok_or_err(), Ok(udec128!(0.333333333333333333333333333333333333333)));
```

### Arithmetic policy

[ArithmeticPolicy]: #arithmetic-policy

Arithmetic policy is an "environment" defines the rules for unwrapping the result of an arithmetic operation containing
emergency flags to get the result:

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
in debug mode it panics on overflow, and in release mode it performs `Saturating` if possible.

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

So when needed, the programmer has full control over rounding, overflow and other emergency flags handling. This
includes an option to enforce exact arithmetic by using `panic!` to block any inexact operations.

### Compare and ordering

The result of any compare operation is always exact and unrounded.

|       | Rust operator |                     Unsigned                      |                  Signed                   |
|-------|:-------------:|:-------------------------------------------------:|:-----------------------------------------:|
| eq    |     `==`      |    [`eq`](crate::decimal::UnsignedDecimal::eq)    |    [`eq`](crate::decimal::Decimal::eq)    |
| ne    |     `!=`      |    [`ne`](crate::decimal::UnsignedDecimal::ne)    |    [`ne`](crate::decimal::Decimal::ne)    |
| lt    |      `<`      |    [`lt`](crate::decimal::UnsignedDecimal::lt)    |    [`lt`](crate::decimal::Decimal::lt)    |
| le    |     `<=`      |    [`le`](crate::decimal::UnsignedDecimal::le)    |    [`le`](crate::decimal::Decimal::le)    |
| gt    |      `>`      |    [`gt`](crate::decimal::UnsignedDecimal::gt)    |    [`gt`](crate::decimal::Decimal::gt)    |
| ge    |     `>=`      |    [`ge`](crate::decimal::UnsignedDecimal::ge)    |    [`ge`](crate::decimal::Decimal::ge)    |
| cmp   |       ➖       |   [`cmp`](crate::decimal::UnsignedDecimal::cmp)   |   [`cmp`](crate::decimal::Decimal::cmp)   |
| max   |       ➖       |   [`max`](crate::decimal::UnsignedDecimal::max)   |   [`max`](crate::decimal::Decimal::max)   |
| min   |       ➖       |   [`min`](crate::decimal::UnsignedDecimal::min)   |   [`min`](crate::decimal::Decimal::min)   |
| clamp |       ➖       | [`clamp`](crate::decimal::UnsignedDecimal::clamp) | [`clamp`](crate::decimal::Decimal::clamp) |

#### Examples

```
use fastnum::udec256;

let n = udec256!(0.1) + udec256!(0.1) + udec256!(0.1) - udec256!(0.3);
assert_eq!(n.to_string(), "0");
assert!(n.is_zero());
```

### Rust operators overloads

[RustOperatorsOverloads]: #rust-operators-overloads

Common numerical operations (such as addition operator `+`, addition assignment operator `+=`, division operator
`/`, [etc...](https://doc.rust-lang.org/book/appendix-02-operators.html)) are overloaded for `fastnum` decimals, so we
can treat them the same way we treat other numbers.

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

**Note**: All Rust overloaded operators uses [`unwrap`](decimal::DecimalResult::unwrap) method of DecimalResult. So the
decision about whether to ignore the overflow and underflow flags and whether the rounded or wrapped result is used or
not is decided based on [DefaultArithmeticPolicy].

## Signed zero

Signed zero is zero with an associated sign. In ordinary arithmetic, the number `0` does not have a sign, so that `−0`,
`+0` and `0` are equivalent. However, in computing, some number representations allow for the existence of two zeros,
often denoted by `−0` (negative zero) and `+0` (positive zero), regarded as equal by the numerical comparison operations
but with possible different behaviors in particular operations. This occurs in the sign-magnitude and ones' complement
signed number representations for integers, and in most floating-point number representations. The number `0` is usually
encoded as `+0`, but can still be represented by `+0`, `−0`, or `0`.

Real arithmetic with signed zeros can be considered a variant of the extended real number line such that `1/−0 = −∞` and
`1/+0 = +∞`; division is undefined only for `±0/±0` and `±∞/±∞`.

Negatively signed zero echoes the mathematical analysis concept of approaching `0` from below as a one-sided limit,
which may be denoted by `x → 0−`, `x → +0`. The notation `−0` may be used informally to denote a negative number that
has been rounded to zero. The concept of negative zero also has some theoretical applications in statistical mechanics
and other disciplines.

More about [Signed Zero](https://en.wikipedia.org/wiki/Signed_zero).

## Normalization

|               |                          Unsigned                           |                       Signed                        |
|---------------|:-----------------------------------------------------------:|:---------------------------------------------------:|
| normalization | [`normalized`](crate::decimal::UnsignedDecimal::normalized) | [`normalized`](crate::decimal::Decimal::normalized) |

## Rescaling

|            |                          Unsigned                           |                       Signed                        |
|------------|:-----------------------------------------------------------:|:---------------------------------------------------:|
| with_scale | [`with_scale`](crate::decimal::UnsignedDecimal::with_scale) | [`with_scale`](crate::decimal::Decimal::with_scale) |

## Rounding

Rounding is applied when a result coefficient has more significant digits than the value of precision; in this case the
result coefficient is shortened to precision digits and may then be incremented by one (which may require a further
shortening), depending on the rounding algorithm selected and the remaining digits of the original coefficient. The
exponent is adjusted to compensate for any shortening.

When a result is rounded, the coefficient may become longer than the current precision. In this case the least
significant digit of the coefficient (it will be a zero) is removed (reducing the precision by one), and the exponent is
incremented by one. This in turn may give rise to an overflow condition, which determines the result after overflow.

### Rounding mode

[RoundingMode]: #rounding-mode

[RoundingMode](crate::decimal::RoundingMode) enum determines how to calculate the last digit of the number when
performing rounding:

| Mode       | Description                                                                    | Default |
|------------|--------------------------------------------------------------------------------|:-------:|
| `Up`       | Always round away from zero.                                                   |         |
| `Down`     | Always round towards zero.                                                     |         |
| `Ceiling`  | Round towards +∞.                                                              |         |
| `Floor`    | Round towards -∞.                                                              |         |
| `HalfUp`   | Round to 'nearest neighbor', or up if ending decimal is `5`.                   |    ✅    |
| `HalfDown` | Round to 'nearest neighbor', or down if ending decimal is `5`.                 |         |
| `HalfEven` | Round to 'nearest neighbor', if equidistant, round towards nearest even digit. |         |

#### Up

Round away from zero. If all the discarded digits are zero the result is unchanged. Otherwise, the result coefficient
should be incremented by `1` (rounded up).

* `5.5` → `6.0`
* `2.5` → `3.0`
* `1.6` → `2.0`
* `1.1` → `2.0`
* `-1.1` → `-2.0`
* `-1.6` → `-2.0`
* `-2.5` → `-3.0`
* `-5.5` → `-6.0`

#### Down

Round toward zero, truncate. The discarded digits are ignored; the result is unchanged.

* `5.5` → `5.0`
* `2.5` → `2.0`
* `1.6` → `1.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-1.0`
* `-2.5` → `-2.0`
* `-5.5` → `-5.0`

#### Ceiling

Round toward +∞. If all the discarded digits are zero or if the sign is `1` the result is unchanged. Otherwise, the
result coefficient should be incremented by `1` (rounded up).

* `5.5` → `6.0`
* `2.5` → `3.0`
* `1.6` → `2.0`
* `1.1` → `2.0`
* `-1.1` → `-1.0`
* `-1.6` → `-1.0`
* `-2.5` → `-2.0`
* `-5.5` → `-5.0`

#### Floor

Round toward -∞. If all the discarded digits are zero or if the sign is `0` the result is unchanged. Otherwise, the
sign is `1` and the result coefficient should be incremented by `1`.

* `5.5` → `5.0`
* `2.5` → `2.0`
* `1.6` → `1.0`
* `1.1` → `1.0`
* `-1.1` → `-2.0`
* `-1.6` → `-2.0`
* `-2.5` → `-3.0`
* `-5.5` → `-6.0`

#### HalfUp

If the discarded digits represent greater than or equal to half (`0.5`) of the value of a one in the next left position
then the result coefficient should be incremented by `1` (rounded up). Otherwise, the discarded digits are ignored.

* `5.5` → `6.0`
* `2.5` → `3.0`
* `1.6` → `2.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-2.0`
* `-2.5` → `-3.0`
* `-5.5` → `-6.0`

#### HalfDown

If the discarded digits represent greater than half (`0.5`) of the value of a one in the next left position then the
result coefficient should be incremented by `1` (rounded up). Otherwise (the discarded digits are `0.5` or less) the
discarded digits are ignored.

* `5.5` → `5.0`
* `2.5` → `2.0`
* `1.6` → `2.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-2.0`
* `-2.5` → `-2.0`
* `-5.5` → `-5.0`

#### HalfEven

If the discarded digits represent greater than half (`0.5`) the value of a one in the next left position then the result
coefficient should be incremented by `1` (rounded up). If they represent less than half, then the result coefficient is
not adjusted (that is, the discarded digits are ignored).

Otherwise (they represent exactly half) the result coefficient is unaltered if its rightmost digit is even, or
incremented by `1` (rounded up) if its rightmost digit is odd (to make an even digit).

* `5.5` → `6.0`
* `2.5` → `2.0`
* `1.6` → `2.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-2.0`
* `-2.5` → `-2.0`
* `-5.5` → `-6.0`

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

| Feature           | Default | Description                                                                                                                                                                                         |
|-------------------|:-------:|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `std`             |    ✅    |                                                                                                                                                                                                     |
| `libm`            |         | Must be used with `no-std` configuration.                                                                                                                                                           |
| `numtraits`       |         | Includes implementations of traits from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/) crate.                                                                                    |
| `rand`            |         | Allows creation of random `fastnum` decimals via the [`rand`](https://docs.rs/rand/latest/rand/) crate.                                                                                             |
| `zeroize`         |         | Enables the [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) trait implementation from the [`zeroize`](https://docs.rs/zeroize/latest/zeroize/) crate for `fastnum` decimals. |
| `serde`           |         | Enables serialization and deserialization of `fastnum` decimals via the [`serde`](https://docs.rs/serde/latest/serde/) crate.                                                                       |
| `diesel`          |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) crate.                                                                        |
| `diesel_postgres` |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) PostgreSQL backend.                                                           |
| `diesel_mysql`    |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) MySQL backend.                                                                |
| `sqlx`            |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) crate.                                                                              |
| `sqlx_postgres`   |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) PostgreSQL backend.                                                                 |
| `sqlx_mysql`      |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) MySQL backend.                                                                      |
| `utoipa`          |         | Enables support of `fastnum` decimals for autogenerated OpenAPI documentation via the [`utoipa`](https://docs.rs/utoipa/latest/utoipa/) crate.                                                      |

### Compile-Time Configuration

You can set a few default parameters at _compile-time_ via environment variables:

| Environment Variable                           | Default  |
|------------------------------------------------|----------|
| `RUST_FASTNUM_DEFAULT_ROUNDING_MODE`           | `HalfUp` |
| `RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD` | `5`      |
| `RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD` | `15`     |
| `RUST_FASTNUM_FMT_MAX_INTEGER_PADDING`         | `1000`   |
| `RUST_FASTNUM_SERDE_DESERIALIZE_MODE`          | `Strict` |
