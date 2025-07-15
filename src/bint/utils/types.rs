use crate::bint::intrinsics::ExpType;

macro_rules! decode_float {
    ($name: ident, $f: ty, $u: ty) => {
        #[inline(always)]
        pub const fn $name(f: $f) -> ($u, i16) {
            const BITS: u32 = core::mem::size_of::<$f>() as u32 * 8;
            const MANT_MASK: $u = <$u>::MAX >> (BITS - (<$f>::MANTISSA_DIGITS - 1));
            const EXP_MASK: $u = <$u>::MAX >> 1;
            const BIAS: i16 = <$f>::MAX_EXP as i16 - 1;

            let bits = f.to_bits();
            let exp = ((bits & EXP_MASK) >> (<$f>::MANTISSA_DIGITS - 1)) as i16;
            let mut mant = bits & MANT_MASK;
            if exp != 0 {
                mant |= (1 << (<$f>::MANTISSA_DIGITS - 1));
            }
            (mant, exp - (BIAS + <$f>::MANTISSA_DIGITS as i16 - 1))
        }
    };
}

decode_float!(decode_f32, f32, u32);
decode_float!(decode_f64, f64, u64);

#[inline(always)]
pub const fn u32_bits(u: u32) -> ExpType {
    32 - u.leading_zeros() as ExpType
}

#[inline(always)]
pub const fn u64_bits(u: u64) -> ExpType {
    64 - u.leading_zeros() as ExpType
}
