use pheno::Fitness;

use crate::genetics::pheno;

macro_rules! implement_fitness_int {
    ( $($t:ty),* ) => {
        $(
            impl Fitness for $t {
                fn zero() -> $t {
                    0
                }

                fn abs_diff(&self, other: &$t) -> $t {
                    if self > other {
                        self - other
                    } else {
                        other - self
                    }
                }
            }
        )*
    }
}

implement_fitness_int!(i8, i16, i32, i64, u8, u16, u32, u64, usize);
