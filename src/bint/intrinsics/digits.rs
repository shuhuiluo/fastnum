use crate::bint::intrinsics::Digits;

#[inline]
pub const fn last_digit_index<const N: usize>(digits: &Digits<N>) -> usize {
    let mut index = 0;
    let mut i = 1;

    while i < N {
        if digits[i] != 0 {
            index = i;
        }
        i += 1;
    }
    index
}
