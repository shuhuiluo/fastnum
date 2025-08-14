pub(crate) trait Widen {}
pub(crate) trait Narrow {}

pub(crate) struct Dimension<const N: usize, const M: usize>;

// This is unstable https://users.rust-lang.org/t/generic-const-exprs-in-trait-bounds/132951/4
// #[cfg(nightly)]
// mod dimensions {
//     use super::*;
//
//     struct Condition<const U: bool>;
//     trait IsTrue {}
//     impl IsTrue for Condition<true> {}
//
//     impl<const N: usize, const M: usize> Widen for Dimension<N, M>
//     where
//         Condition<{ N >= M }>: IsTrue
//     {}
//
//     impl<const N: usize, const M: usize> Narrow for Dimension<N, M>
//     where
//         Condition<{ N < M }>: IsTrue
//     {}
// }
// #[cfg(not(nightly))]
mod dimensions {
    use super::*;

    macro_rules! dim {
        ([$N: literal, $($T: literal),*]) => {
            $(
                impl Widen for Dimension<$T, $N> {}

                impl Narrow for Dimension<$N, $T> {}
            )*

            dim!([$($T),*]);
        };
        ([$N: literal]) => {};
    }

    dim!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
}
