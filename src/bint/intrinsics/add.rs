use crate::bint::intrinsics::Digit;

#[inline]
pub const fn _carrying_add_64(a: Digit, b: Digit, carry: bool) -> (Digit, bool) {
    let (s1, o1) = a.overflowing_add(b);
    if carry {
        let (s2, o2) = s1.overflowing_add(1);
        (s2, o1 || o2)
    } else {
        (s1, o1)
    }
}
