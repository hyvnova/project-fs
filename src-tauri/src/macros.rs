#[macro_export]
/// Creates an map of `function_name`:`function pointer` given `function name`s
/// Ex. `define_function_names(foo, bar, gold)` 
/// Translates to
/// ```
/// FUNCTION_MAP = {
///     function_name: <pointer to function>, *
/// }
/// ``` 
macro_rules! define_functions {
    ( $( $fn_name:ident ),* ) => {
        $(
            use crate::functions::$fn_name;
        )*


        lazy_static::lazy_static! {
            pub static ref FUNCTION_MAP: std::collections::HashMap<String, crate::types::FunctionTy> = {
                let mut m = std::collections::HashMap::new();
                $(
                    m.insert(stringify!($fn_name).to_string(), $fn_name as crate::types::FunctionTy);
                )*
                m
            };
        }
    };
}
