macro_rules! opt_format {    
    ($ident:expr, $fmt:expr) => (
        opt_format!($ident, $fmt, )
    );
    
    ($ident:expr, $fmt:expr, $($arg:expr),*) => (
        $ident.map(|item| format!($fmt, item, $($arg,)*))
    );
}
macro_rules! opt_as_str {
    ($ident:expr) => (
        $ident.as_ref().map(String::as_str)
    )
}