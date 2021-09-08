#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

#[cfg_attr(test, macro_use)]
extern crate float_cmp;

#[macro_export]
macro_rules! assert_tensor_f32_eq {
    ($a: expr, $b: expr) => {
        assert!(
            approx_eq!(&Tensor<f32>, $a, $b),
            "assertion failed: `(left == right)`\n  left: `{:?}`,\n right: `{:?}`\n",
            $a,
            $b
        )
    };
}

mod core;
mod creation;
mod ops;
mod random;
mod util;

pub use crate::core::*;
pub use util::*;
