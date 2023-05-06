

//MACRO match_chain_or_return!
/// Recursively match one or more patterns, each with optional guard clauses,
/// and resolve to the final expresion if all matches succeed, else return the fail expression
/// to the enclosing function
/// 
/// The syntax is as follows:
/// ```ignore
/// match_chain_or_return!(
///   fail_binding, fail_return_expression;
///   expr_to_match, pattern (| optional_patterns) (optional_guard_clause) =>
///        ...         ...            ...                   ...
///   expr_to_match, pattern (| optional_patterns) (optional_guard_clause) => final_expr
/// )
/// ```
/// ### Example
/// ```rust
/// use grit_junk_drawer::match_chain_or_return;
/// 
/// type Complex = Option<Result<Option<Result<u64,()>>,()>>;
/// 
/// fn main() -> Result<(), String> {
///     let my_complex: Complex = Some(Ok(Some(Ok(42))));
///     let my_simple = Some(69);
/// 
///     let my_complex_val = match_chain_or_return!(
///         _, Err(String::from("Error"));
///         my_complex, Some(res_1) =>
///         res_1, Ok(opt_2) =>
///         opt_2, Some(res_2) =>
///         res_2, Ok(val) if val == 42 => val
///     );
/// 
///     let my_simple_val = match_chain_or_return!(
///         _, Err(String::from("Error"));
///         my_simple, Some(val) if val == 69 => val
///     );
/// 
///     assert_eq!(my_complex_val, 42);
///     assert_eq!(my_simple_val, 69);
///     
///     // can also bind a variable in the fail path
///     let return_from_fail = (|| match_chain_or_return!(
///         fail_var, Err(fail_var);
///         my_simple, Some(val) if val == 100 => Ok(val)
///     ))();
/// 
///     assert_eq!(return_from_fail, Err(Some(69)));
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! match_chain_or_return {
    ($fail_ident:ident, $fail_expr:expr; $expression:expr, $( $pattern:pat_param )|+ $( if $guard: expr )? => $pass_expr:expr) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => $pass_expr,
            $fail_ident => return $fail_expr
        }
    };
    (_, $fail_expr:expr; $expression:expr, $( $pattern:pat_param )|+ $( if $guard: expr )? => $pass_expr:expr) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => $pass_expr,
            _ => return $fail_expr
        }
    };
    ($fail_ident:ident, $fail_expr:expr; $expression:expr, $( $pattern:pat_param )|+ $( if $guard: expr )? => $($tail:tt)*) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => match_chain_or_return!($fail_expr; $($tail)*),
            $fail_ident => return $fail_expr
        }
    };
    (_, $fail_expr:expr; $expression:expr, $( $pattern:pat_param )|+ $( if $guard: expr )? => $($tail:tt)*) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => match_chain_or_return!(_, $fail_expr; $($tail)*),
            _ => return $fail_expr
        }
    };
}
