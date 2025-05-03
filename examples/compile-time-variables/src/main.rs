use fastnum::*;

fn main() {
    assert_eq!((dec128!(1000) / dec128!(94000)).round(2), dec128!(0.01));
}
