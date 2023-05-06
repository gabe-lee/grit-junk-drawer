

//MACRO match_or_return!
/// Match one or more patterns with an optional guard clause
/// - If matched, pass the inner expresion back
/// - Otherwise return the specified return expresion to the enclosing function
/// 
/// Similar to [matches!] but instead of resolving to a boolean, it resolves to the inner expression on a match,
/// or else returns from the enclosing function with the return expresion
/// ```rust
/// use grit_junk_drawer::match_or_return;
/// 
/// fn main() -> Result<(), String> {
///     let my_enum = Some(42);
/// 
///     let my_val = match_or_return!(my_enum, Some(val) if val < 50, val, Err(String::from("Error")));
/// 
///     assert_eq!(my_val, 42);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! match_or_return {
    ($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?, $inner:expr, $return:expr) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => $inner,
            _ => return $return
        }
    };
}
