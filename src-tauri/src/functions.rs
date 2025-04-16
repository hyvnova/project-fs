use crate::types::Arg;

/// Returns true if source contains target
pub fn has<'a, 'b>(source: &'a str, args: &Arg) -> bool {
    match args {
        Arg::Literal(arg) => {
            return source.contains(arg)
        }
        _ => {return false ;}
    }
}