macro_rules! consts_impl {
    () => {
        impl<const N: usize> Decimal<N> {
            
            /// **N**ot **A** **N**umber value. More about [`NaN`](crate#special-values).
            pub const NAN: Self = Self::new(UInt::ZERO, 0, ControlBlock::nan());
            
            /// Infinity (∞). More about [`±Infinity`](crate#special-values).
            pub const INFINITY: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::infinity());
            
            /// Negative infinity (−∞). More about [`±Infinity`](crate#special-values).
            pub const NEG_INFINITY: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::neg_infinity());
            
            /// The smallest value that can be represented by this decimal type - (2<sup>N</sup> - 1) × 10<sup>32'768</sup>.
            pub const MIN: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::default().neg());
            
            /// The maximum value that this type can represent (2<sup>N</sup> - 1) × 10<sup>32'768</sup>.
            pub const MAX: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::default());
    
            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);
            
            /// Euler's number (e).
            pub const E: Self = math::consts::Consts::<N>::E;
            
            /// Archimedes’ constant (π).
            pub const PI: Self = math::consts::Consts::<N>::PI;
            
            /// The full circle constant (τ)
            ///
            /// Equal to 2π.
            pub const TAU: Self = math::consts::Consts::<N>::TAU;
            
            /// 1/π.
            pub const FRAC_1_PI: Self = math::consts::Consts::<N>::FRAC_1_PI;
            
            /// 2/π.
            pub const FRAC_2_PI: Self = math::consts::Consts::<N>::FRAC_2_PI;
            
            /// π/2.
            pub const FRAC_PI_2: Self = math::consts::Consts::<N>::FRAC_PI_2;
            
            /// π/3.
            pub const FRAC_PI_3: Self = math::consts::Consts::<N>::FRAC_PI_3;
            
            /// π/4.
            pub const FRAC_PI_4: Self = math::consts::Consts::<N>::FRAC_PI_4;
            
            /// π/6.
            pub const FRAC_PI_6: Self = math::consts::Consts::<N>::FRAC_PI_6;
            
            /// π/8.
            pub const FRAC_PI_8: Self = math::consts::Consts::<N>::FRAC_PI_8;
            
            /// 2/sqrt(π).
            pub const FRAC_2_SQRT_PI: Self = math::consts::Consts::<N>::FRAC_2_SQRT_PI;
            
            /// ln(2).
            pub const LN_2: Self = math::consts::Consts::<N>::LN_2;
            
            /// ln(10).
            pub const LN_10: Self = math::consts::Consts::<N>::LN_10;
            
            /// log<sub>2</sub>(e).
            pub const LOG2_E: Self = math::consts::Consts::<N>::LOG2_E;
            
            /// log<sub>10</sub>(e).
            pub const LOG10_E: Self = math::consts::Consts::<N>::LOG10_E;
            
            /// sqrt(2).
            pub const SQRT_2: Self = math::consts::Consts::<N>::SQRT_2;
            
            /// 1/sqrt(2).
            pub const FRAC_1_SQRT_2: Self = math::consts::Consts::<N>::FRAC_1_SQRT_2;
            
            /// log<sub>10</sub>(2).
            pub const LOG10_2: Self = math::consts::Consts::<N>::LOG10_2;
            
            /// log<sub>2</sub>(10).
            pub const LOG2_10: Self = math::consts::Consts::<N>::LOG2_10;
            
            /// E<sub>max</sub> = 32'768
            pub(crate) const E_MAX: i32 = -(i16::MIN as i32);
            
            /// E<sub>min</sub> = -32'767
            pub(crate) const E_MIN: i32 = -(i16::MAX as i32);
            
            pub(crate) const COEFF_MAX: UInt<N> = UInt::<N>::MAX;
            
            pub(crate) const COEFF_MEDIUM: UInt<N> = Self::COEFF_MAX.div(UInt::<N>::TEN);
            
            pub(crate) const COEFF_MEDIUM_PLUS_ONE: UInt<N> = Self::COEFF_MEDIUM.strict_add(UInt::ONE);
            
            /// Max length of the _coefficient_ in decimal digits.
            pub(crate) const MAX_CLENGTH: i32 = math::utils::clength(UInt::<N>::MAX);
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(UInt::$name, 0, ControlBlock::default());
        )*
    }
}

pub(crate) use consts_impl;