use fastnum::*;

fn main() {
    println!("Hello, fastnum!");

    // Integers

    let uint64 = u64!(1234567890);
    println!("U64: {uint64}");

    let uint128: U128 = uint64.cast();
    println!("U128: {uint128}");

    let uint1024: U1024 = uint128.cast();
    println!("U1024: {uint1024}");

    let int1024: I1024 = uint1024.try_cast().unwrap();
    println!("I1024: {int1024}");

    let uint512: U512 = int1024.try_cast().unwrap();
    println!("U512: {uint512}");

    let uint64: U64 = uint512.try_cast().unwrap();
    println!("U64: {uint64}");

    let uint64 = u64!(18446744073709551615);
    let int64 = i64!(-9223372036854775808);

    assert!(<U64 as TryCast<I64>>::try_cast(uint64).is_err());
    assert_eq!(<U64 as Cast<I128>>::cast(uint64), i128!(18446744073709551615));
    assert!(<I64 as TryCast<U64>>::try_cast(int64).is_err());

    // Decimals

    let udec64 = udec64!(12345.67890);
    println!("UD64: {udec64}");

    let udec128: UD128 = udec64.cast();
    println!("UD128: {udec128}");

    let dec128: D128 = udec128.cast();
    println!("D128: {dec128}");

    let udec512: UD512 = dec128.try_cast().unwrap();
    println!("U512: {udec512}");

    let udec64: UD64 = udec512.try_cast().unwrap();
    println!("U64: {udec64}");
}
