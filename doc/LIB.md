# fastnum

Fixed-size signed and unsigned decimal numbers implemented in pure Rust. Suitable for
financial, crypto and any other fixed-precision calculations.

[IEEE 754]: https://en.wikipedia.org/wiki/IEEE_754

[IEEE 854]: https://en.wikipedia.org/wiki/IEEE_854-1987

## Overview

`fastnum` provides signed and unsigned exact precision decimal numbers suitable for financial calculations that
require significant integral and fractional digits with no round-off errors (such as 0.1 + 0.2 ≠ 0.3).

Any `fastnum` decimal type consists of an N-bit big unsigned integer, paired with a 64-bit signaling block which
contains a 16-bit scaling factor determines the position of the decimal point, sign, special, and signaling flags.
Trailing zeros are preserved and may be exposed when in string form.

Thus, `fastnum` decimal numbers are trivially copyable and do not require any dynamic allocation. This allows you to get
additional performance gains by eliminating not only dynamic allocation, like such, but also will get rid of one
indirect addressing, which improves cache-friendliness and reduces the CPU load.

### Why fastnum?

- **Strictly exact precision**: no round-off errors (such as 0.1 + 0.2 ≠ 0.3).
- **Special values**: `fastnum` support `±0`, `±Infinity` and `NaN` special values.
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
fastnum = "0.1"
```

Or, to enable various `fastnum` features as well, add for example this line instead:

```toml
fastnum = { version = "0.1", features = ["serde"] } # enables the "serde" feature
```

## Example usage

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
use fastnum::{udec256, UD256, decimal::Context};

// This value will be evaluated at compile-time and inlined directly into the relevant context when used. 
const PI_X_2: UD256 = udec256!(2).mul(udec256!(3.141592653589793115997963468544185161590576171875), Context::default());
```

Compile-time checks

```compile_fail
use fastnum::{udec256, UD256};

// Invalid character.
const E: UD256 = udec256!(A3.5);
```

```compile_fail
use fastnum::{udec256, UD256, decimal::Context};

// Arithmetic error during calculation.
const E: UD256 = udec256!(1.5).div(udec256!(0), Context::default());
```

## Representation

### Abstract model

Numbers represent the values which can be manipulated by, or be the results of.
Numbers may be finite numbers (numbers whose value can be represented exactly) or they may be special
values (infinities and other values which are not finite numbers).

#### Finite numbers

The numerical value of a finite number is given by: (–1)<sup>sign</sup> × coefficient × 10<sup>exponent</sup>.

- **_sign_** – a value which must be either `0` or `1`, where `1` indicates that the number is negative or is the
  negative zero and `0` indicates that the number is zero or positive.
- **_coefficient_** – an unsigned integer which is zero or positive.
- **_exponent_** – a signed integer which indicates the power of ten by which the coefficient is multiplied.

For example, if the sign had the value `1`, the exponent had the value `–1`, and the coefficient had the value `25`,
then the numerical value of the number is exactly `–2.5`.

Trailing zeros are preserved and may be exposed when in string form. These can be truncated using the normalize or
round functions.

A number with a coefficient of `0` is permitted to have both `+` and `-` sign.
Negative zero is accepted as an operand for all operations (see [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)).

The _quantum_ of a finite number is given by: 1 × 10<sup>-exp</sup>. This is the value of a unit in the
least significant position of the _coefficient_ of a finite number.

This abstract definition deliberately allows for multiple representations of values which are numerically equal but are
visually distinct (such as `1` and `1.00`).
However, there is a one-to-one mapping between the abstract representation and the result of the primary conversion to
string using to-scientific-string on that abstract representation.
In other words, if one number has a different abstract representation to another, then the primary string conversion
will also be different.

#### Exponent

The _exponent_ is represented by a two’s complement 16-bit binary integer.

The **_adjusted exponent_** is the value of the exponent of a number when that number is expressed as though in
scientific notation with one digit (non-zero unless the coefficient is `0`) before any decimal point.
This is given by the value of the _exponent + (C<sub>length</sub> – 1)_, where _C<sub>length</sub>_ is the length of the
coefficient in decimal digits.
When a limit to the exponent _E<sub>limit</sub>_ applies,
it must result in a balanced range of positive or negative numbers, taking into account the magnitude of the
coefficient.
To achieve this balanced range, the minimum and maximum values of the adjusted exponent (_E<sub>min</sub>_ and _E<sub>
max</sub>_ respectively) must have magnitudes which differ by no more than one, so _E<sub>min</sub>_ will be _–E<sub>
max</sub> ±1_.
[IEEE 754] further constrains this so that _E<sub>min</sub> = 1 – E<sub>max</sub>_.
Therefore, if the length of the coefficient is _C<sub>length</sub>_ digits, the exponent may take any of the values
_-E<sub>limit</sub> – (C<sub>length</sub> – 1) + 1_ through _E<sub>limit</sub> – (C<sub>length</sub> – 1)_.

#### Special values

[Infinity]: #special-values

[NaN]: #special-values

In addition to the finite numbers, numbers must also be able to represent one of three named special values:

- `±Infinity` – a value representing a number whose magnitude is infinitely large (see [IEEE 754] §3.2 and §6.1).
- `quiet NaN` – a value representing undefined results (_Not a Number_) which does not cause an Invalid operation
  condition.
- `signaling NaN` – a value representing undefined results (_Not a Number_) which will usually cause an Invalid
  operation condition if used in any operation defined in this specification (see [IEEE 754] §3.2 and §6.2).

When a number has one of these special values, its coefficient and exponent are undefined.
All special values may have a sign, as for finite numbers.
The sign of an `Infinity` is significant (that is, it is possible to have both positive and negative infinity), and the
sign of a `NaN` has no meaning.

#### Signed zero

[Zero]: #signed-zero

Signed zero (`±0`) is zero with an associated sign.
In ordinary arithmetic, the number `0` does not have a sign, so that `−0`, `+0` and `0` are equivalent.
However, in computing, some number representations allow for the existence of two zeros, often denoted by `−0` (negative
zero) and `+0` (positive zero), regarded as equal by the numerical comparison operations but with possible different
behaviors in particular operations.

Real arithmetic with signed zeros can be considered a variant of the extended real number line such that `1/−0 = −∞` and
`1/+0 = +∞`; division is undefined only for `±0/±0` and `±∞/±∞`.

Negatively signed zero echoes the mathematical analysis concept of approaching `0` from below as a one-sided limit,
which may be denoted by `x → 0−`, `x → +0`. The notation `−0` may be used informally to denote a negative number that
has been rounded to zero. The concept of negative zero also has some theoretical applications in statistical mechanics
and other disciplines.

More about [Signed Zero](https://en.wikipedia.org/wiki/Signed_zero).

#### Normal numbers, subnormal numbers, and Underflow

[Subnormal]: #normal-numbers-subnormal-numbers-and-underflow

In any context where exponents are bounded most finite numbers are normal.
Non-zero finite numbers whose adjusted exponents are greater than or equal to _E<sub>min</sub>_ are called normal numbers;
those non-zero numbers whose adjusted exponents are less than _E<sub>min</sub>_ are called subnormal numbers.
Like other numbers, subnormal numbers are accepted as operands for all operations, and may result from any operation.
If a result is subnormal, before any rounding, then the Subnormal condition is raised.

For a subnormal result, the minimum value of the exponent becomes _E<sub>min</sub> – (precision – 1)_, called _E<sub>
tiny</sub>_, where precision is the working precision, as described below.
The result will be rounded, if necessary, to ensure that the exponent is no smaller than _E<sub>tiny</sub>_.
If, during this rounding, the result becomes inexact, then the Underflow condition is raised.
A subnormal result does not necessarily raise Underflow, therefore, but is always indicated by the Subnormal condition (
even if, after rounding, its value is `0` or ten to the power of _E<sub>min</sub>_).

When a number underflows to zero during a calculation, its exponent will be _E<sub>tiny</sub>_. 
The maximum value of the exponent is unaffected.

Note that the minimum value of the exponent for subnormal numbers is the same as the minimum value of exponent which can
arise during operations which do not result in subnormal numbers, which occurs in the case where C<sub>length</sub> =
precision.

### Memory layout

Under the hood any N-bit decimal type consists of an N-bit big unsigned integer (_coefficient_), paired with a _control
block_ which consists of the following components:

- a signed 16-bit integer scaling factor (negotiated _exponent_),
- number flags (include _sign_ and special values flags),
- signaling flags.

<style>
.mermaid svg { 
  max-width: 100% !important; 
  height: auto;
}
</style>
<pre class="mermaid">
    ---
    title: D128 decimal memory layout (bytes)
    config:
      theme: 'forest'
      packet:
        bitsPerRow: 40
        bitWidth: 18
    ---
    packet-beta
    0-31: "coefficient (U128)"
    32-33: "exp"
    34-34: "f"
    35-39: "context"
</pre>
<pre class="mermaid">
    ---
    title: D128 decimal memory layout (bits)
    config:
        theme: 'forest'
        packet:
            bitsPerRow: 192
            bitWidth: 5
    ---
    packet-beta
    0-127: "coefficient (U128)"
    128-143: "exp"
    144-151: "flags"
    152-191: "context"
</pre>

<script type="module">
import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
var doc_theme = localStorage.getItem("rustdoc-theme");
if (doc_theme === "dark" || doc_theme === "ayu") 
    mermaid.initialize({theme: "dark", rough: true});
</script>

#### Signaling flags and trap-enablers

The exceptional conditions, which may arise during performing arithmetic operations on decimal numbers, are grouped into
signals, which can be controlled individually.

Signaling flags are very similar to the [processor flag register](https://en.wikipedia.org/wiki/FLAGS_register) and
contain information about arithmetic errors such as operation overflow, division by `0`, or loss of precision
calculations.
Depending on the needs of the application, flags may be ignored, considered as informational, or cause `panic!`.

The signals are:

|     **Signal**      | Description                                                                                      |
|:-------------------:|--------------------------------------------------------------------------------------------------|
|      `CLAMPED`      | The exponent of a result has been altered.                                                       |
| `DIVISION_BY_ZERO`  | Non-zero dividend is divided by zero.                                                            |
|      `INEXACT`      | Result is not exact (one or more non-zero coefficient digits were discarded during rounding).    |                                                                                    
| `INVALID_OPERATION` | Invalid operation. Result would be undefined or impossible.                                      |
|     `OVERFLOW`      | Indicates that exponent of the decimal result is too large to fit the target type.               |
|      `ROUNDED`      | Result has been rounded (that is, some zero or non-zero coefficient digits were discarded).      |
|     `SUBNORMAL`     | Result is _subnormal_ (its adjusted exponent is less than E<sub>min</sub>), before any rounding. |
|     `UNDERFLOW`     | Result is both subnormal and inexact.                                                            |

For each of the signals, the corresponding flag in _signals block_ is set to 1 when the signal occurs.
For each of the signals, the corresponding Context _trap-enabler_ indicates which action is to be taken when the signal
occurs (see [IEEE 754] §7). If `0`, a defined result is supplied, and execution continues (for example, an overflow is
perhaps converted to a positive or negative infinity). If `1`, then execution of the operation cause `panic!`.

##### Clamped

This occurs and signals clamped if the exponent of a result has been altered in order to fit the constraints of the
underlying `i16` integer type.
This may occur when the exponent of a zero result would be outside the bounds of a
representation, or (in the [IEEE 754] interchange formats) when a large normal number would have an encoded exponent
that can't be represented.
In this latter case, the exponent is reduced to fit and the corresponding number of zero
digits are appended to the coefficient (“fold-down”).
The condition always occurs when a subnormal value rounds to zero.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = dec256!(1E-10);
    let b = dec256!(1E-100);
    a + b
});

assert_eq!(res, dec256!(1E-10));
assert!(res.is_op_inexact());
assert!(res.is_op_rounded());
assert!(res.is_op_clamped());

```

##### Division by zero

This occurs and signals division-by-zero if division of a finite number by zero was attempted, and the dividend was not
zero.
The result of the operation is `±Infinity`, where sign is the exclusive or of the signs of the operands for divide, or
is `1` for an odd power of `-0`, for power.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = dec256!(1);
    let b = dec256!(0);
    a / b
});

assert!(res.is_infinite());
assert!(res.is_op_div_by_zero());
```

##### Inexact

This occurs and signals inexact whenever the result of an operation is not exact (that is, it needed to be rounded and
any discarded digits were non-zero), or if an overflow or underflow condition occurs.
The result in all cases is unchanged.

For example, `1/3 = 0.333333333333(3)`. The result of division is an infinite decimal fraction which cannot
be stored in any of the existing types: performing the operation will cause `OVERFLOW` for any finite count of
decimal digits However, the result can be obtained if we make a deal and agree to the loss of precision.
In this case, the result will be the rounded value, accompanied by information that rounding has been performed and
the result may not be accurate.

|         | Result            | Rounded (HalfUp) | Rounded (Down) | `INEXACT` |
|---------|-------------------|------------------|----------------|:---------:|
| `6 / 3` | 2                 | 2                | 2              |           |
| `1 / 3` | 0.333333333333(3) | 0.333333333333   | 0.333333333333 |    ⁉️     |
| `2 / 3` | 0.666666666666(6) | 0.666666666667   | 0.666666666666 |    ⁉️     |

For most practical purposes this is an acceptable trade-off. However, we always leave the possibility
be sure that the result is accurate and there were no loss of precision.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = dec128!(1);
    let b = dec128!(3);
    a / b
});

assert_eq!(res, dec128!(0.333333333333333333333333333333333333333));
assert!(res.is_op_inexact());
```

##### Invalid operation

This occurs and signals invalid-operation if:

- an operand to an operation is `NaN`.
- an attempt is made to add `+Infinity` to `-Infinity` during an addition or subtraction operation.
- an attempt is made to multiply `0` by `±Infinity`.
- an attempt is made to divide either `+Infinity` or `-Infinity` by either `+Infinity` or `-Infinity`.
- the divisor for a remainder operation is zero.
- the dividend for a remainder operation is `±Infinity`.
- either operand of the quantize operation is infinite.
- the operand of the `ln` or the `log10` operation is less than zero.
- the operand of the square-root operation has a negative sign and a non-zero coefficient.
- both operands of the power operation are zero, or if the left-hand operand is less than zero and the right-hand
  operand does not have an integral value or is infinite.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = D128::INFINITY;
    let b = D128::INFINITY;
    a - b
});

assert!(res.is_nan());
assert!(res.is_op_invalid());
```

##### Overflow

This occurs and signals overflow if the adjusted exponent of a result (from a conversion or from an operation that is
not an attempt to divide by zero), after rounding, would be greater than the largest value that can be handled by the
implementation (the value E<sub>max</sub>).
The result depends on the rounding mode:

- For round-half-up and round-half-even (and for round-half-down and round-up), the result of the operation is
  `±Infinity`, where sign is the sign of the intermediate result.
- For round-down, (and round-05up, if implemented), the result is the largest finite number that can be represented in
  the current precision, with the sign of the intermediate result.
- For round-ceiling, the result is the same as for round-down if the sign of the intermediate result is `-`, or is
  `Infinity` otherwise.
- For round-floor, the result is the same as for round-down if the sign of the intermediate result is `+`, or is
  `-Infinity` otherwise.

In all cases, `Inexact` and `Rounded` will also be raised.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = D128::MAX;
    let b = D128::MAX;
    a * b
});

assert!(res.is_infinite());
assert!(res.is_op_overflow());
assert!(res.is_op_rounded());
assert!(res.is_op_inexact());
```

##### Rounded

This occurs and signals rounded whenever the result of an operation is rounded (that is, some zero or non-zero digits
were discarded from the coefficient), or if an overflow or underflow condition occurs.
The result in all cases is unchanged.
The rounded signal may be tested (or trapped) to determine if a given operation (or sequence of operations) caused a
loss of precision.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = D128::MAX;
    let b = dec128!(1.0);
    a * b
});

assert_eq!(res, D128::MAX);
assert!(res.is_op_rounded());
```

##### Subnormal

This occurs and signals subnormal whenever the result of a conversion or operation is subnormal (that is, its adjusted
exponent is less than E<sub>min</sub>, before any rounding).
The result in all cases is unchanged. The subnormal signal may be tested (or trapped) to determine if a given or
operation (or sequence of operations) yielded a subnormal result.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = dec128!(1E-30000);
    let b = dec128!(1E2768);
    a / b
});

assert!(res.is_op_subnormal());
```

##### Underflow

This occurs and signals underflow if a result is inexact and the adjusted exponent of the result would be smaller (more
negative) than the smallest value that can be handled by the implementation (the value E<sub>min</sub>).
That is, the result is both inexact and subnormal.
The result after an underflow will be a subnormal number rounded, if necessary, so that its exponent is not less than
E<sub>tiny</sub>.
This may result in `0` with the sign of the intermediate result and an exponent of E<sub>tiny</sub>.

In all cases, [`Inexact`](#inexact), [`Rounded`](#rounded), and [`Subnormal`](#subnormal) will also be raised.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = with_context!(ctx, {
    let a = dec128!(1e-32767);
    let b = D128::MAX;
    a / b
});

assert!(res.is_op_underflow());
assert!(res.is_op_inexact());
assert!(res.is_op_rounded());
assert!(res.is_op_subnormal());

```

## Precision

Precision is an integral number of decimal digits. It is limited by maximum decimal digits that N-bit unsigned
coefficient can hold.

## Arithmetic

The `fastnum` crate provides support for fast correctly rounded decimal floating-point arithmetic. It offers several
advantages over the native floating-point ([IEEE 754]) types:

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
assert_eq!(n.to_string(), "0.0");
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

8. [Context](#decimal-context) defines the rules for unwrapping the result of an arithmetic operation containing
   [signaling flags](#signaling-flags-and-trap-enablers).

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
assert_eq!(n.to_string(), "0.0");
assert!(n.is_zero());
```

### Rust operators overloads

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

Unfortunately the current version of Rust does not support const traits, so this example fail to compile:

```compile_fail
use fastnum::{udec256, UD256};

const A: UD256 = udec256!(3.5);
const B: UD256 = udec256!(2.5);
const C: UD256 = A + B;
```

In constant calculations and static contexts, until the [`feature`](https://github.com/rust-lang/rust/issues/67792) is
stabilized, the following const methods should be used:

```
use fastnum::{udec256, UD256, decimal::Context};

const A: UD256 = udec256!(3.5);
const B: UD256 = udec256!(2.5);
const C: UD256 = A.add(B, Context::default());

assert_eq!(C, udec256!(6));
```

**Note**: all Rust overloaded operators use default [Decimal Context](#default-context).
So the decision about whether to ignore the overflow and underflow flags and whether the rounded or wrapped result is
used or not is decided based on [Default Context](#default-context).

## Decimal context

**_Decimal context_** represents the user-selectable parameters and rules which govern the results of arithmetic
operations (for example, the rounding mode when rounding occurs).

The context is defined by the following parameters:

- `rounding_mode`: a named value which indicates the algorithm to be used when rounding is necessary, see more
  about [rounding mode](#rounding-mode);
- `signal_traps`: For each of the signals, the corresponding trap-enabler indicates which action is to be taken when the
  signal occurs (see [IEEE 754] §7). 
  See more about [Signals](#signaling-flags-and-trap-enablers).

### Default context

|     **Signal**      | Trap-enabler |
|:-------------------:|:------------:|
|      `CLAMPED`      |              |
| `DIVISION_BY_ZERO`  |      ⚠️      |
|      `INEXACT`      |              |                                                                                    
| `INVALID_OPERATION` |      ⚠️      |
|     `OVERFLOW`      |      ⚠️      |
|      `ROUNDED`      |              |
|     `SUBNORMAL`     |              |
|     `UNDERFLOW`     |              |

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

### Default display

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
fastnum = { version = "0.1", features = ["serde"] } 
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

### Compile-time configuration

You can set a few default parameters at _compile-time_ via environment variables:

| Environment Variable                           | Default  |
|------------------------------------------------|----------|
| `RUST_FASTNUM_DEFAULT_ROUNDING_MODE`           | `HalfUp` |
| `RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD` | `5`      |
| `RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD` | `15`     |
| `RUST_FASTNUM_FMT_MAX_INTEGER_PADDING`         | `1000`   |
| `RUST_FASTNUM_SERDE_DESERIALIZE_MODE`          | `Strict` |
