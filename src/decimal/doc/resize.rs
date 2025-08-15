macro_rules! resize {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Safety resizes the underlying decimal to use `M` limbs while preserving the numeric value when possible.\n"
            "This operation can either widen or narrow the internal representation:"
            "- Widening (`M >= N`) is lossless: the value is preserved."
            "- Narrowing (`M < N`) may reduce available capacity. In this case the"
            "  value is rounded according to the current [`Context`] and"
            "  corresponding status flags are set.\n"

            "Behavior details:\n"
            "- Rounding: extra precision is rounded using the active [`RoundingMode`]"
            "  from the current context."
            "- Signals: status flags such as `Inexact`, `Rounded`, `Clamped`,"
            "  `Overflow`, or `Underflow` may be raised depending on the operation"
            "  outcome and context limits.\n"

            "Note: lossless, no-rounding conversions.\n"
            "If you need to change width without any rounding:"
            "- Use [`crate::Cast`] for guaranteed-lossless widening (value-preserving by"
            "  definition).\n"
            "- Use [`crate::TryCast`] for potential narrowing without rounding; it returns"
            "  an error if the value does not fit into the target width, thus"
            "  guaranteeing no silent rounding or truncation.\n"

            "# Performance\n"
            "This operation is typically much slower than [crate::Cast] and [crate::TryCast] transformations.\n",

            #Panics
            doc::decimal_operation_panics!("resize operation"),

            #Also
            "[crate::Cast]"
            "[crate::TryCast]",

            #Examples("Lossless widening:")
            ["let x = " doc::m!(64 $($sign)?) "(123.45);"]
            [""]
            ["// Increase internal width from 2 to 4 limbs — value is preserved."]
            ["let y: " doc::type_str!(128 $($sign)?) " = x.resize();"]
            ["assert_eq!(y, " doc::m!(128 $($sign)?) "(123.45));"]
            ["assert!(y.is_op_ok());"],

            #Examples("Narrowing with possible rounding:")
            ["let x = " doc::m!(128 $($sign)?) "(1.8446744073709551616);"]
            [""]
            ["// Reduce width; value may be rounded according to context."]
            ["let y: " doc::type_str!(64 $($sign)?) " = x.resize();"]
            ["// Rounding/precision-loss indicators may be set, depending on capacity and context:"]
            ["assert_eq!(y, " doc::m!(64 $($sign)?) "(1.844674407370955162));"]
            ["assert!(y.is_op_inexact() && y.is_op_rounded());"]
        }
    };
}

pub(crate) use resize;
