macro_rules! count_ones {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #count_ones,
            $sign $bits,
            "Returns the number of ones in the binary representation of `self`.",

            "let a = " doc::m!($sign $bits) "(7);\n\n"
            "assert_eq!(a.count_ones(), 3);\n"
        }
    };
}

pub(crate) use count_ones;

macro_rules! count_zeros {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #count_zeros,
            $sign $bits,
            "Returns the number of zeros in the binary representation of `self`.",

            "let a = " doc::type_str!($sign $bits) "::" doc::num::count_zeros!(@ $sign) ";\n\n"
            "assert_eq!(a.count_zeros(), 0);\n"
        }
    };
    (@ U) => {
        "MAX"
    };
    (@ I) => {
        "NEG_ONE"
    };
}

pub(crate) use count_zeros;

macro_rules! leading_zeros {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #leading_zeros,
            $sign $bits,
            "Returns the number of leading zeros in the binary representation of `self`.\n\n"
            "Depending on what you're doing with the value, you might also be interested in the\n"
            "[Self::ilog2] function which returns a consistent number, even if the type widens.",

            "let a = " doc::type_str!($sign $bits) "::" doc::num::leading_zeros!(@ $sign) ";\n\n"
            "assert_eq!(a.leading_zeros(), 0);\n"
        }
    };
    (@ U) => {
        "MAX"
    };
    (@ I) => {
        "MIN"
    };
}

pub(crate) use leading_zeros;

macro_rules! trailing_zeros {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #trailing_zeros,
            $sign $bits,
            "Returns the number of trailing zeros in the binary representation of `self`.",

            "let a = " doc::m!($sign $bits) "(4);\n\n"
            "assert_eq!(a.trailing_zeros(), 2);\n"
        }
    };
}

pub(crate) use trailing_zeros;

macro_rules! leading_ones {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #leading_ones,
            $sign $bits,
            "Returns the number of leading ones in the binary representation of `self`.\n\n",

            "let a = " doc::type_str!($sign $bits) "::" doc::num::leading_ones!(@ $sign) ";\n\n"
            "assert_eq!(a.leading_ones(), " stringify!($bits) ");\n"
        }
    };
    (@ U) => {
        "MAX"
    };
    (@ I) => {
        "NEG_ONE"
    };
}

pub(crate) use leading_ones;

macro_rules! trailing_ones {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #trailing_ones,
            $sign $bits,
            "Returns the number of trailing ones in the binary representation of `self`.",

            "let a = " doc::m!($sign $bits) "(3);\n\n"
            "assert_eq!(a.trailing_ones(), 2);\n"
        }
    };
}

pub(crate) use trailing_ones;

macro_rules! cast_unsigned {
    ($bits: literal) => {
        doc::doc_comment! {
            #cast_unsigned,
            I $bits,
            "Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.\n\n"
            "This produces the same result as an `as` cast, but ensures that the bit-width remains the same.",

            "let a = " doc::m!(I $bits) "(-1);\n\n"
            "assert_eq!(a.cast_unsigned(), " doc::type_str!(U $bits) "::MAX);\n"
        }
    };
}

pub(crate) use cast_unsigned;

macro_rules! cast_signed {
    ($bits: literal) => {
        doc::doc_comment! {
            #cast_signed,
            U $bits,
            "Returns the bit pattern of `self` reinterpreted as an unsigned integer of the same size.\n\n"
            "This produces the same result as an `as` cast, but ensures that the bit-width remains the same.",

            "let a = " doc::type_str!(U $bits) "::MAX;\n\n"
            "assert_eq!(a.cast_signed(), " doc::m!(I $bits)  "(-1));\n"
        }
    };
}

pub(crate) use cast_signed;

macro_rules! rotate_left {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #rotate_left,
            $sign $bits,
            "Shifts the bits to the left by a specified amount, `n`,\n"
            "wrapping the truncated bits to the end of the resulting integer.\n\n"
            "Please note this isn't the same operation as the `<<` shifting operator!"
        }
    };
}

pub(crate) use rotate_left;

macro_rules! rotate_right {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #rotate_right,
            $sign $bits,
            "Shifts the bits to the left by a specified amount, `n`, \n"
            "wrapping the truncated bits to the end of the resulting integer.\n\n"
            "Please note this isn't the same operation as the `>>` shifting operator!\n"
            "`self.rotate_right(n)` is equivalent to `self.rotate_left(Self::BITS - n)`."
        }
    };
}

pub(crate) use rotate_right;

macro_rules! swap_bytes {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #swap_bytes,
            $sign $bits,
            "Reverses the byte order of the integer.",

            "let n = " doc::m!($sign $bits) "(0x12345678901234567890123456789012);\n"
            "assert_eq!(n.swap_bytes().swap_bytes(), n);\n"
        }
    };
}

pub(crate) use swap_bytes;

macro_rules! reverse_bits {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #reverse_bits,
            $sign $bits,
            "Reverses the order of bits in the integer.\n\n"
            "The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.",

            "let n = " doc::m!($sign $bits) "(0x12345678901234567890123456789012);\n"
            "assert_eq!(n.reverse_bits().reverse_bits(), n);\n"
        }
    };
}

pub(crate) use reverse_bits;

macro_rules! pow {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #pow,
            $sign $bits,
            "Raises `self` to the power of `exp`, using exponentiation by squaring.",

            "let n = " doc::m!($sign $bits) "(3);\n"
            "assert_eq!(n.pow(5), " doc::m!($sign $bits) "(243));\n"
        }
    };
}

pub(crate) use pow;

macro_rules! add {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `+` operation."
        }
    };
}

pub(crate) use add;

macro_rules! mul {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `*` operation."
        }
    };
}

pub(crate) use mul;

macro_rules! shl {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `<<` operation."
        }
    };
}

pub(crate) use shl;

macro_rules! shr {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `>>` operation."
        }
    };
}

pub(crate) use shr;

macro_rules! sub {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `-` operation."
        }
    };
}

pub(crate) use sub;

macro_rules! div {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `/` operation."
        }
    };
}

pub(crate) use div;

macro_rules! rem {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `%` operation."
        }
    };
}

pub(crate) use rem;

macro_rules! div_euclid {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #div_euclid,
            $sign $bits,
            "Performs Euclidean division.\n\n"
            "Since, for the positive integers, all common definitions of division are equal, this is exactly equal to self / rhs.\n\n"

            "# Panics\n\n"
            "This function will panic if rhs is zero."
        }
    };
}

pub(crate) use div_euclid;

macro_rules! rem_euclid {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #rem_euclid,
            $sign $bits,
            "Calculates the least remainder of self (mod rhs).\n\n"
            "Since, for the positive integers, all common definitions of division are equal, this is exactly equal to self % rhs.\n\n"

            "# Panics\n\n"
            "This function will panic if rhs is zero."
        }
    };
}

pub(crate) use rem_euclid;

macro_rules! is_power_of_two {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #is_power_of_two,
            $sign $bits,
            "Returns `true` if and only if `self == 2^k` for some integer `k`.",

            "let n = " doc::m!($sign $bits) "(8);\n"
            "assert!(n.is_power_of_two());\n"
            "let m = " doc::m!($sign $bits) "(90);\n"
            "assert!(!m.is_power_of_two());\n"
        }
    };
}

pub(crate) use is_power_of_two;

macro_rules! midpoint {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #midpoint,
            $sign $bits,
            "Calculates the midpoint (average) between self and rhs.\n\n"
            "midpoint(a, b) is (a + b) / 2 as if it were performed in a sufficiently-large unsigned integral type.\n"
            "This implies that the result is always rounded towards zero and that no overflow will ever occur."
        }
    };
}

pub(crate) use midpoint;

macro_rules! ilog2 {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #ilog2,
            $sign $bits,
            "Returns the base 2 logarithm of the number, rounded down.\n\n"

            "# Panics\n\n"
            "This function will panic if self is zero."
        }
    };
}

pub(crate) use ilog2;

macro_rules! ilog10 {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #ilog10,
            $sign $bits,
            "Find integer log<sub>10</sub>(x) of an integer.\n\n"

            "`fastnum` use the most efficient algorithm based on relationship:\n"
            "_log<sub>10</sub>(x) = log<sub>2</sub>(x)/log<sub>2</sub>(10)_,\n"
            "we can compute the _log<sub>10</sub>(x)_ as `ilog2(x)` multiplied by\n"

            "_1/log<sub>2</sub>(10)_, which is approximately `1233/4096`, or `1233`"
            "followed by a right shift of `12`."
            "_((`ilog2`(x) + 1) * 1233) >> 12_\n\n"
            "Adding one is needed because the `ilog2()` rounds down. Finally, since the"
            "resulting value is only an approximation that may be off by one, the exact"
            "value is found by subtracting `1` if `x < PowersOf10[res]` (lookup table)."
            "This method takes `6` more operations than `ilog2()`.",

            "let n = " doc::m!($sign $bits) "(150);\n"
            "assert_eq!(n.ilog10(), 2);\n"
        }
    };
}

pub(crate) use ilog10;

macro_rules! ilog {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #ilog,
            $sign $bits,
            "Returns the logarithm of the number with respect to an arbitrary base, rounded down.\n\n"
            "This method might not be optimized owing to implementation details;\n"
            "ilog2 can produce results more efficiently for base 2, and ilog10 can produce results more efficiently for base 10."

            "# Panics\n\n"
            "This function will panic if self is zero, or if base is less than 2."
        }
    };
}

pub(crate) use ilog;

macro_rules! abs_diff {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #abs_diff,
            $sign $bits,
            "Computes the absolute difference between self and other."
        }
    };
}

pub(crate) use abs_diff;

macro_rules! next_multiple_of {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #next_multiple_of,
            $sign $bits,
            "Calculates the smallest value greater than or equal to self that is a multiple of rhs.\n\n"

            "# Panics\n\n"
            "This function will panic if rhs is zero.\n\n"

            "## Overflow behavior\n\n"
            "On overflow, this function will panic if overflow checks are enabled (default in debug mode) and wrap if overflow checks are disabled (default in release mode).\n\n"
        }
    };
}

pub(crate) use next_multiple_of;

macro_rules! div_floor {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #div_floor,
            $sign $bits,
            "Calculates the quotient of self and rhs, rounding the result towards negative infinity.\n\n"

            "# Panics\n\n"
            "This function will panic if rhs is zero.\n\n"
        }
    };
}

pub(crate) use div_floor;

macro_rules! div_ceil {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            #div_ceil,
            $sign $bits,
            "Calculates the quotient of self and rhs, rounding the result towards positive infinity.\n\n"

            "# Panics\n\n"
            "This function will panic if rhs is zero.\n\n"
        }
    };
}

pub(crate) use div_ceil;

macro_rules! bits {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns the smallest number of bits necessary to represent `self`.\n\n"
            "This is equal to the size of the type in bits minus the leading zeros of `self`.",

            "assert_eq!(" doc::m!($sign $bits) "(0b1111001010100).bits(), 13);\n"
            "assert_eq!(" doc::type_str!($sign $bits) "::ZERO.bits(), 0);\n"
        }
    };
}

pub(crate) use bits;

macro_rules! bit {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns a boolean representing the bit in the given position (`true` if the bit is set).\n\n"
            "The least significant bit is at index `0`, the most significant bit is at index `Self::BITS - 1`.",

            "let n = " doc::m!($sign $bits) "(0b001010100101010101);\n"
            "assert!(n.bit(0));\n"
            "assert!(!n.bit(1));\n"
            "assert!(!n.bit(" doc::type_str!($sign $bits) "::BITS - 1));\n"
        }
    };
}

pub(crate) use bit;

macro_rules! power_of_two {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns an integer whose value is `2^power`. This is faster than using a shift left on `Self::ONE`.\n\n"

            "# Panics\n\n"
            "This function will panic if `power` is greater than or equal to `Self::BITS`.",

            "let n = " doc::type_str!($sign $bits) "::power_of_two(11);\n"
            "assert_eq!(n, (1u128 << 11).try_into().unwrap());\n"
        }
    };
}

pub(crate) use power_of_two;

macro_rules! digits {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns the digits stored in `self` as an array.\n"
            "Digits are little endian (least significant digit first)."
        }
    };
}

pub(crate) use digits;

macro_rules! digits_mut {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns the digits stored in `self` as a mutable array.\n"
            "Digits are little endian (least significant digit first)."
        }
    };
}

pub(crate) use digits_mut;

macro_rules! from_digits {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Creates a new unsigned integer from the given array of digits.\n"
            "Digits are stored as little endian (least significant digit first)."
        }
    };
}

pub(crate) use from_digits;

macro_rules! from_digit {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Creates a new unsigned integer from the given digit.\n"
            "The given digit is stored as the least significant digit."
        }
    };
}

pub(crate) use from_digit;

macro_rules! div_rem {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Simultaneous truncated integer division and modulus.\n\n"
            "Returns `(quotient, remainder)`.",

            "assert_eq!(" doc::m!($sign $bits) "(8).div_rem(" doc::m!($sign $bits) "(3)), (" doc::m!($sign $bits) "(2), " doc::m!($sign $bits) "(2)));\n"
        }
    };
}

pub(crate) use div_rem;

macro_rules! div_rem_digit {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Simultaneous truncated integer division and modulus.\n\n"
            "Returns `(quotient, remainder)`."
        }
    };
}

pub(crate) use div_rem_digit;

macro_rules! mul_div_rem {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the `self * rhs / divisor` operation.\n\n"
            "Returns `(quotient, remainder)`."
        }
    };
}

pub(crate) use mul_div_rem;

macro_rules! neg {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Performs the unary `-` operation."
        }
    };
}

pub(crate) use neg;

macro_rules! from_bits {
    ($bits: literal) => {
        doc::doc_comment! {
            I $bits,
            "Creates an integer with bits as its underlying representation in two's complement."
        }
    };
}

pub(crate) use from_bits;

macro_rules! to_bits {
    ($bits: literal) => {
        doc::doc_comment! {
            I $bits,
            "This simply returns the underlying representation of the integer in two's complement, as an unsigned integer."
        }
    };
}

pub(crate) use to_bits;

macro_rules! unsigned_abs {
    ($bits: literal) => {
        doc::doc_comment! {
            #unsigned_abs,
            I $bits,
            "Computes the absolute value of self as unsigned integer without panicking."

            "let a = " doc::m!(I $bits) "(-50);\n"
            "let b = " doc::m!(U $bits) "(50);\n\n"
            "assert_eq!(a.unsigned_abs(), b);\n"
        }
    };
}

pub(crate) use unsigned_abs;

macro_rules! abs {
    ($bits: literal) => {
        doc::doc_comment! {
            #abs,
            I $bits,
            "Computes the absolute value of self.\n\n"

            "## Overflow behavior\n\n"
            "The absolute value of i128::MIN cannot be represented as an i128, and attempting to calculate it will cause an overflow. This means that code in debug mode will trigger a panic on this case and optimized code will return i128::MIN without a panic. If you do not want this behavior, consider using unsigned_abs instead.\n\n"

            "let a = " doc::m!(I $bits) "(-50);\n"
            "let b = " doc::m!(I $bits) "(50);\n\n"
            "assert_eq!(a.abs(), b);\n"
        }
    };
}

pub(crate) use abs;

macro_rules! signum {
    ($bits: literal) => {
        doc::doc_comment! {
            #signum,
            I $bits,
            "Returns a number representing sign of `self`.\n\n"
            "  - `0` if the number is zero\n"
            "  - `1` if the number is positive\n"
            "  - `-1` if the number is negative",

            "assert_eq!(" doc::m!(I $bits) "(10).signum(), " doc::m!(I $bits) "(1));\n"
            "assert_eq!(" doc::m!(I $bits) "(0).signum(), " doc::m!(I $bits) "(0));\n"
            "assert_eq!(" doc::m!(I $bits) "(-10).signum(), " doc::m!(I $bits) "(-1));\n"
        }
    };
}

pub(crate) use signum;

macro_rules! is_positive {
    ($bits: literal) => {
        doc::doc_comment! {
            #is_positive,
            I $bits,
            "Returns true if self is positive and false if the number is zero or negative."
        }
    };
}

pub(crate) use is_positive;

macro_rules! is_negative {
    ($bits: literal) => {
        doc::doc_comment! {
            #is_negative,
            I $bits,
            "Returns true if self is negative and false if the number is zero or positive."
        }
    };
}

pub(crate) use is_negative;

macro_rules! power_of_ten {
    ($sign: ident $bits: literal) => {
        doc::doc_comment! {
            $sign $bits,
            "Returns an integer whose value is 10^power.\n\n"

            "# Panics\n\n"
            "This function will panic if `10^power` is greater than [Self::MAX]",

            "assert_eq!(" doc::type_str!($sign $bits) "::power_of_ten(2), " doc::m!($sign $bits) "(100));\n"
        }
    };
}

pub(crate) use power_of_ten;
