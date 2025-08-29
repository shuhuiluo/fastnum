macro_rules! with_ctx {
    ($bits: literal $($sign: ident)?) => {
        doc::doc_comment! {
            $bits $($sign)?,

            "Apply new [Context] to the given decimal number.\n"
            "Returns a copy of the value with the provided context applied.\n"
            "This method updates the operational context (including the rounding mode and"
            "other contextual parameters) used by subsequent operations that may round or"
            "clamp the value (e.g., add, sub, mul, div, round, rescale, quantize, etc.).\n"

            "**Important:**"
            "- The change is local to the returned value and does not affect other values."
            "- If you ignore the returned value, the context update is lost."
            "- If the value currently carries extra precision, that extra precision is"
            "  reconciled immediately with the new context: it is rounded using the"
            "  context’s rounding mode (and may be clamped as dictated by the context)."
            "- If the current value already has signaling flags set (e.g., [`INEXACT`](crate#inexact) and the"
            "  new context enables traps for those signals, applying this method may trigger"
            "  the corresponding traps immediately, which can result in a panic (depending"
            "  on the build/configuration).",

            #Panics
            doc::decimal_panics!("\n - the current value already has some signaling flags set (e.g., [`INEXACT`](crate#inexact)) \
            and the new [Context] enables traps for those signals;\n- or possible extra precision rounding operation \
            performs with some [Exceptional condition](crate#signaling-flags-and-trap-enablers) \
            and the new [Context] enables traps for those [Exceptional condition](crate#signaling-flags-and-trap-enablers)."),

            #Also
            "More about [`Decimal context`](crate#decimal-context) decimals."
            "More about [`Exceptional conditions`](crate#signaling-flags-and-trap-enablers)."
            "[Context]"
            "[RoundingMode]",

            #Examples
            ["let ctx = decimal::Context::default().without_traps();"]
            ["let a = " doc::m!($bits $($sign)?) "(1).with_ctx(ctx);"]
            ["let b = " doc::m!($bits $($sign)?) "(0).with_ctx(ctx);"]
            [""]
            ["// No panic! We can divide by zero!"]
            ["let c = a / b;"]
            ["assert!(c.is_infinite());"]
            ["assert!(c.is_op_div_by_zero());"]
        }
    };
}

pub(crate) use with_ctx;
