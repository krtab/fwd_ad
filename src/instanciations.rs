//! A module containing various instanciations of canonical owning/view `Dual` pairs.
//!
//! Each submodule contains to typedefs: `Owning` and `View`.
pub mod vecf64 {
    use super::super::*;
    pub type Owning = Dual<Vec<f64>, RW, f64>;
    pub type View<'a> = Dual<&'a [f64], RO, f64>;
}

pub mod vecf32 {
    use super::super::*;
    pub type Owning = Dual<Vec<f32>, RW, f32>;
    pub type View<'a> = Dual<&'a [f32], RO, f32>;
}

// TODO
macro_rules! inst_array {
    ($modname : ident, $ftype: ty, $n : literal) => {
        pub mod $modname {
            use super::super::*;
            pub type Owning = Dual<[$ftype; $n], RW, $ftype>;
            pub type View<'a> = Dual<&'a [$ftype; $n], RO, $ftype>;
        }
    };
}

inst_array!(arr_f32_1, f32, 1);
inst_array!(arr_f32_2, f32, 2);
inst_array!(arr_f32_3, f32, 3);
inst_array!(arr_f32_4, f32, 4);
inst_array!(arr_f32_5, f32, 5);
inst_array!(arr_f32_6, f32, 6);
inst_array!(arr_f32_7, f32, 7);
inst_array!(arr_f32_8, f32, 8);
inst_array!(arr_f32_9, f32, 9);
inst_array!(arr_f32_10, f32, 10);
inst_array!(arr_f32_11, f32, 11);
inst_array!(arr_f32_12, f32, 12);
inst_array!(arr_f32_13, f32, 13);
inst_array!(arr_f32_14, f32, 14);
inst_array!(arr_f32_15, f32, 15);
inst_array!(arr_f32_16, f32, 16);
inst_array!(arr_f32_17, f32, 17);
inst_array!(arr_f32_18, f32, 18);
inst_array!(arr_f32_19, f32, 19);
inst_array!(arr_f32_20, f32, 20);
inst_array!(arr_f32_21, f32, 21);
inst_array!(arr_f32_22, f32, 22);
inst_array!(arr_f32_23, f32, 23);
inst_array!(arr_f32_24, f32, 24);
inst_array!(arr_f32_25, f32, 25);
inst_array!(arr_f32_26, f32, 26);
inst_array!(arr_f32_27, f32, 27);
inst_array!(arr_f32_28, f32, 28);
inst_array!(arr_f32_29, f32, 29);
inst_array!(arr_f32_30, f32, 30);
inst_array!(arr_f32_31, f32, 31);
inst_array!(arr_f32_32, f32, 32);
inst_array!(arr_f64_1, f64, 1);
inst_array!(arr_f64_2, f64, 2);
inst_array!(arr_f64_3, f64, 3);
inst_array!(arr_f64_4, f64, 4);
inst_array!(arr_f64_5, f64, 5);
inst_array!(arr_f64_6, f64, 6);
inst_array!(arr_f64_7, f64, 7);
inst_array!(arr_f64_8, f64, 8);
inst_array!(arr_f64_9, f64, 9);
inst_array!(arr_f64_10, f64, 10);
inst_array!(arr_f64_11, f64, 11);
inst_array!(arr_f64_12, f64, 12);
inst_array!(arr_f64_13, f64, 13);
inst_array!(arr_f64_14, f64, 14);
inst_array!(arr_f64_15, f64, 15);
inst_array!(arr_f64_16, f64, 16);
inst_array!(arr_f64_17, f64, 17);
inst_array!(arr_f64_18, f64, 18);
inst_array!(arr_f64_19, f64, 19);
inst_array!(arr_f64_20, f64, 20);
inst_array!(arr_f64_21, f64, 21);
inst_array!(arr_f64_22, f64, 22);
inst_array!(arr_f64_23, f64, 23);
inst_array!(arr_f64_24, f64, 24);
inst_array!(arr_f64_25, f64, 25);
inst_array!(arr_f64_26, f64, 26);
inst_array!(arr_f64_27, f64, 27);
inst_array!(arr_f64_28, f64, 28);
inst_array!(arr_f64_29, f64, 29);
inst_array!(arr_f64_30, f64, 30);
inst_array!(arr_f64_31, f64, 31);
inst_array!(arr_f64_32, f64, 32);
