use fastnum::*;

/// Example output
///
/// ```text
/// 1.0
/// 2.0
/// 3.0
/// 3.0
/// 4.0
/// 5.0
/// ```
fn main() {
    let mut positions = vec![
        dec128!(2.0),
        dec128!(4.0),
        dec128!(1.0),
        dec128!(5.0),
        dec128!(3.0),
        dec128!(3.0),
    ];

    positions.sort_by(|a, b| a.cmp(&b));

    for i in positions {
        println!("{i}");
    }
}
