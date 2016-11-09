macro_rules! opt_format {    
    ($ident:expr, $fmt:expr) => (
        opt_format!($ident, $fmt, )
    );
    
    ($ident:expr, $fmt:expr, $($arg:expr),*) => (
        $ident.map(|item| Cow::from(format!($fmt, item, $($arg,)*)))
    );
}