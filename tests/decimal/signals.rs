use rstest::*;

use fastnum::{decimal::*, *};

#[rstest(::trace)]
fn test_div_by_zero() {
    let ctx = Context::default().without_traps();

    // No panic! We can divide by zero!
    let res = dec256!(1.0).with_ctx(ctx) / dec256!(0).with_ctx(ctx);

    assert!(res.is_infinite());
    assert!(res.is_op_div_by_zero());
    assert!(res.is_op_invalid());
}

#[rstest(::trace)]
fn test_x() {
    let a = dec512!(1.41);
    let b = dec512!(3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481);
    let c = dec512!(-1.731592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481);
    
    assert_eq!(a - b, c);
    assert_eq!(a - c, b);
    assert_eq!(b + c, a);
}

