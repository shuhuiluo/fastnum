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
    
}
