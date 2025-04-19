use std::fs;
use std::path::Path;

use crate::types::{ Arg, Operator, Unit };
use chrono::NaiveDate;

/// Returns true if source contains target
pub fn has<'a>(source: &'a str, args: &Arg) -> Result<bool, String> {
    match args {
        Arg::Literal(arg) => {
            // Check for regex syntax (e.g., /pattern/)
            if arg.starts_with('\\') && arg.ends_with('\\') && arg.len() > 2 {
                let pattern = &arg[1..arg.len() - 1];
                return Ok(
                    regex::Regex
                        ::new(pattern)
                        .map(|re| re.is_match(source))
                        .unwrap_or(false)
                );
            }

            Ok(source.contains(arg))
        }

        _ => Err("Invalid argument. expected conditional.".to_string()),
    }
}

pub fn size<'a, 'b>(source: &'a str, args: &Arg) -> Result<bool, String> {
    let path = Path::new(source);

    // Skip if not a file -- tecnically never true, but whatever
    if !path.is_file() {
        return Ok(false);
    }

    let Ok(metadata) = fs::metadata(path) else {
        return Ok(false);
    };

    let file_size = metadata.len(); // in bytes

    match args {
        Arg::Conditional { operator, value } => {
            Ok(match value {
                Unit::Size(target_size) => {
                    match operator {
                        Operator::Gt => file_size > *target_size,
                        Operator::Lt => file_size < *target_size,
                    }
                }
                _ => false,
            })
        }
        Arg::Group(group_args) => {
            // All conditions in group must pass (AND logic)
            for cond in group_args {
                if !size(source, cond)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Err("Invalid argument. expected conditional or group.".to_string()),
    }
}

/// Filter by last modification date
pub fn modified(source: &str, args: &Arg) -> Result<bool, String> {
    let path = Path::new(source);

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return Ok(false),
    };

    let modified_time = match metadata.modified() {
        Ok(time) => time,
        Err(_) => return Ok(false),
    };

    // Convert std::time::SystemTime to chrono::NaiveDate
    let datetime: chrono::DateTime<chrono::Utc> = modified_time.into();
    let naive_date = datetime.date_naive();

    match args {
        Arg::Conditional { operator, value } => {
            match value {
                Unit::Date(target_date) => {
                    Ok(match operator {
                        Operator::Gt => naive_date > *target_date,
                        Operator::Lt => naive_date < *target_date,
                    })
                }
                _ => Err("Invalid conditional: modified expects a date value.".to_string()),
            }
        }
        Arg::Group(group_args) => {
            // AND logic: all conditions must hold
            for cond in group_args {
                if !modified(source, cond)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Err("Invalid argument. Expected conditional or group.".to_string()),
    }
}