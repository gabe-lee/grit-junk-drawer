

//MACRO chain_match!
/// Recursively match one or more patterns, each with optional guard clauses,
/// and resolve to the final expresion or statement if all matches succeed, else resolve to the previous default
/// expression or statement
/// 
/// The syntax is as follows:
/// ```ignore
/// where (default) is literally typing out (default)
/// where <DEFAULT_DEFINITION>     = (default) <pattern>, (<expression>|<statement>);
/// where <MATCH_DEFINITION>       = <expression>, <pattern> (if <expression>)? =>
/// where <FINAL_MATCH_DEFINITION> = <MATCH_DEFINITION> (<expression>|<statement>)
/// 
/// chain_match!(
///   <DEFAULT_DEFINITION>
///   [<DEFAULT_DEFINITION> | <MATCH_DEFINITION>]*
///   <FINAL_MATCH_DEFINITION>
/// )
/// ```
/// ### Examples
/// ```rust
/// use grit_junk_drawer::chain_match;
/// 
/// type Deep = Option<Result<Option<Result<u64,()>>,()>>;
/// type Simple = Option<u64>;
/// type Complex = Option<(bool, Option<bool>)>;
/// 
/// fn main() -> Result<(), String> {
///     let my_deep: Deep = Some(Ok(Some(Ok(42))));
///     let my_simple: Simple = Some(69);
///     let my_complex: Complex = Some((true, Some(false)));
/// 
///     // expression on pass, return static on fail
///     let my_deep_val = chain_match!(
///         (default) _, return Err(format!("expected 42"));
///         my_deep, Some(res_1) =>
///         res_1, Ok(opt_2) =>
///         opt_2, Some(res_2) =>
///         res_2, Ok(val) if val == 42 => val
///     );
///     assert_eq!(my_deep_val, 42);
/// 
///     // expression on pass, return dynamic on fail
///     let my_simple_val = chain_match!(
///         (default) invalid, return Err(format!("expected 69, got {:?}", invalid));
///         my_simple, Some(val) if val == 69 => val
///     );
///     assert_eq!(my_simple_val, 69);
/// 
///     // expression on pass, static expression on fail
///     let my_simple_result = chain_match!(
///         (default) _, Err(format!("expected 70"));
///         my_simple, Some(val) if val == 70 => Ok(val)
///     );
///     assert_eq!(my_simple_result, Err(format!("expected 70")));
/// 
///     // expression on pass, dynamic expression on fail
///     let my_simple_result = chain_match!(
///         (default) invalid, Err(invalid);
///         my_simple, Some(val) if val == 70 => Ok(val)
///     );
///     assert_eq!(my_simple_result, Err(Some(69)));
/// 
///     // change default path midway
///     let my_complex_result = chain_match!(
///         (default) _, Err(format!("First check false"));
///         my_complex, Some((true, next_option)) =>
///         (default) _, Err(format!("Second check false"));
///         next_option, Some(true) => Ok(format!("Both checks pass"))
///     );
///     assert_eq!(my_complex_result, Err(format!("Second check false")));
/// 
///     // execute statement instead of resolving to expression
///     let mut it_passed: Option<bool> = None;
///     chain_match!(
///         (default) _, it_passed = Some(false);
///         my_simple, Some(val) if val == 69 => it_passed = Some(true)
///     );
///     assert_eq!(it_passed, Some(true));
/// 
///     // expression on fail, statement on pass
///     let main_result = chain_match!(
///         (default) _, Err(format!("Program failed"));
///         my_simple, Some(val) if val == 69 => return Ok(())
///     );
/// 
///     return main_result
/// }
/// ```
#[macro_export]
macro_rules! chain_match {
    // Final Match: Fail Expression, Pass Expression
    ((default) $fail_pat:pat, $fail_expr:expr; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => $pass_expr:expr) => {
        match $match_expr {
            $pass_pat $( if $guard )? => $pass_expr,
            $fail_pat => $fail_expr
        }
    };
    // Final Match: Fail Expression, Pass Statement
    ((default) $fail_pat:pat, $fail_expr:expr; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => $pass_stmt:stmt) => {
        match $match_expr {
            $pass_pat $( if $guard )? => $pass_stmt,
            $fail_pat => $fail_expr
        }
    };
    // Final Match: Fail Statement, Pass Expression
    ((default) $fail_pat:pat, $fail_stmt:stmt; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => $pass_expr:expr) => {
        match $match_expr {
            $pass_pat $( if $guard )? => $pass_expr,
            $fail_pat => $fail_stmt
        }
    };
    // Final Match: Fail Statement, Pass Statement
    ((default) $fail_pat:pat, $fail_stmt:stmt; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => $pass_stmt:stmt) => {
        match $match_expr {
            $pass_pat $( if $guard )? => $pass_stmt,
            $fail_pat => $fail_stmt
        }
    };
    // Recursive Match: Fail Expression, New Fail Expression
    ((default) $fail_pat:pat, $fail_expr:expr; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => (default) $new_fail_pat:pat, $new_fail_expr:expr; $($tail:tt)*) => {
        match $match_expr {
            $pass_pat $( if $guard )? => chain_match!((default) $new_fail_pat, $new_fail_expr; $($tail)*),
            $fail_pat => $fail_expr
        }
    };
    // Recursive Match: Fail Expression, New Fail Statement
    ((default) $fail_pat:pat, $fail_expr:expr; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => (default) $new_fail_pat:pat, $new_fail_stmt:stmt; $($tail:tt)*) => {
        match $match_expr {
            $pass_pat $( if $guard )? => chain_match!((default) $new_fail_pat, $new_fail_stmt; $($tail)*),
            $fail_pat => $fail_expr
        }
    };
    // Recursive Match: Fail Expression, Same Fail Expression
    ((default) $fail_pat:pat, $fail_expr:expr; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => $($tail:tt)*) => {
        match $match_expr {
            $pass_pat $( if $guard )? => chain_match!((default) $fail_pat, $fail_expr; $($tail)*),
            $fail_pat => $fail_expr
        }
    };
    // Recursive Match: Fail Statement, New Fail Expression
    ((default) $fail_pat:pat, $fail_stmt:stmt; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => (default) $new_fail_pat:pat, $new_fail_expr:expr; $($tail:tt)*) => {
        match $match_expr {
            $pass_pat $( if $guard )? => chain_match!((default) $new_fail_pat, $new_fail_expr; $($tail)*),
            $fail_pat => $fail_stmt
        }
    };
    // Recursive Match: Fail Statement, New Fail Statement
    ((default) $fail_pat:pat, $fail_stmt:stmt; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => (default) $new_fail_pat:pat, $new_fail_stmt:stmt; $($tail:tt)*) => {
        match $match_expr {
            $pass_pat $( if $guard )? => chain_match!((default) $new_fail_pat, $new_fail_stmt; $($tail)*),
            $fail_pat => $fail_stmt
        }
    };
    // Recursive Match: Fail Statement, Same Fail Statement
    ((default) $fail_pat:pat, $fail_stmt:stmt; $match_expr:expr, $pass_pat:pat $( if $guard:expr )? => $($tail:tt)*) => {
        match $match_expr {
            $pass_pat $( if $guard )? => chain_match!((default) $fail_pat, $fail_stmt; $($tail)*),
            $fail_pat => $fail_stmt
        }
    };
}
