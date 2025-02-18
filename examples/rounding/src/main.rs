use fastnum::*;

fn main() {
    // Ceil
    assert_eq!(3.01_f64.ceil(), 4.0);
    assert_eq!(dec256!(3.01).ceil(), dec256!(4));
    
    assert_eq!(3.99_f64.ceil(), 4.0);
    assert_eq!(dec256!(3.99).ceil(), dec256!(4));
    
    assert_eq!(4.0_f64.ceil(), 4.0);
    assert_eq!(dec256!(4.0).ceil(), dec256!(4));
    
    assert_eq!(1.0001_f64.ceil(), 2.0);
    assert_eq!(dec256!(1.0001).ceil(), dec256!(2));
    
    assert_eq!(1.00001_f64.ceil(), 2.0);
    assert_eq!(dec256!(1.00001).ceil(), dec256!(2));
    
    assert_eq!(1.000001_f64.ceil(), 2.0);
    assert_eq!(dec256!(1.000001).ceil(), dec256!(2));
    
    assert_eq!(1.00000000000001_f64.ceil(), 2.0);
    assert_eq!(dec256!(1.00000000000001).ceil(), dec256!(2));

    assert_eq!((-3.01_f64).ceil(), -3.0);
    assert_eq!(dec256!(-3.01).ceil(), dec256!(-3));
    
    assert_eq!((-3.5_f64).ceil(), -3.0);
    assert_eq!(dec256!(-3.5).ceil(), dec256!(-3));
    
    assert_eq!((-4.0_f64).ceil(), -4.0);
    assert_eq!(dec256!(-4.0).ceil(), dec256!(-4));
    
    // Floor
    assert_eq!(3.99_f64.floor(), 3.0);
    assert_eq!(dec256!(3.99).floor(), dec256!(3));
    
    assert_eq!(3.0_f64.floor(), 3.0);
    assert_eq!(dec256!(3.0).floor(), dec256!(3));
    
    assert_eq!(3.01_f64.floor(), 3.0);
    assert_eq!(dec256!(3.01).floor(), dec256!(3));
    
    assert_eq!(3.5_f64.floor(), 3.0);
    assert_eq!(dec256!(3.5).floor(), dec256!(3));
    
    assert_eq!(4.0_f64.floor(), 4.0);
    assert_eq!(dec256!(4.0).floor(), dec256!(4));
    
    assert_eq!((-3.01_f64).floor(), -4.0);
    assert_eq!(dec256!(-3.01).floor(), dec256!(-4));
    
    assert_eq!((-3.1_f64).floor(), -4.0);
    assert_eq!(dec256!(-3.1).floor(), dec256!(-4));
    
    assert_eq!((-3.5_f64).floor(), -4.0);
    assert_eq!(dec256!(-3.5).floor(), dec256!(-4));
    
    assert_eq!((-4.0_f64).floor(), -4.0);
    assert_eq!(dec256!(-4.0).floor(), dec256!(-4));
}
