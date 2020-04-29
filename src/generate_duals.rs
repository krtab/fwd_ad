/// Generate *n* duals with *n* derivatives, one for each varable.
///
///  Can optionally generate a "getter" closure used to get the derivative *with respect* to the variable.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate smolad;
/// # use smolad::*;
/// # fn main() {
/// generate_duals!{
///     // getdx will get derivatives wrt. x
///     x = 17.; @ getdx
///     // no getter for y
///     y = 42.;
/// }
/// assert_eq!(getdx(x.view()), 1.);
/// assert_eq!(getdx(y.view()), 0.);
/// assert_eq!(x, Dual::<Vec<f64>,RW, f64>::from(vec![17.,1.,0.]));
/// assert_eq!(y, Dual::<Vec<f64>,RW, f64>::from(vec![42.,0.,1.]));
/// # }
/// ```
#[macro_export]
macro_rules! generate_duals {
    {$($varname:ident = $value:expr; $(@ $gettername:ident)?)*} => {
        let ndiffs : usize = [$($value),*].len();
        let mut i : usize = 0;
        $(
            let mut $varname : Dual<Vec<f64>,RW, f64> = Dual::constant($value, ndiffs);
            $varname.diffs_mut()[i] = 1.;

            $(
                let $gettername = move |x : Dual<&[f64],RO, f64>| x.diffs()[i];
            )?

            i+=1;
        )*
    };
}
