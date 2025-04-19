use chrono::NaiveDate;

use crate::{ define_functions, types::{ Arg, Node, Operator, Unit } };

// Creates a map `FUNCTION_MAP`, containing `name`:`function`
// Filter function definition.
define_functions!(has, size, modified);

pub struct Parser;

impl Parser {
    pub fn parse(q: String) -> Vec<Node> {
        q.split_ascii_whitespace()
            .map(|pair| {
                let aux: Vec<&str> = pair.split(":").collect();

                dbg!(&pair);

                if aux.len() != 2 {
                    return Node::Fail(String::new());
                }

                let function = aux[0].trim();
                let raw_args = aux[1].trim();

                if !FUNCTION_MAP.contains_key(function) {
                    return Node::Fail(String::new());
                }

                let args: Arg = match parse_args(raw_args) {
                    Ok(args) => args,
                    Err(e) => {
                        return Node::Fail(e);
                    }
                };

                dbg!(&args);

                return Node::Call {
                    func: match FUNCTION_MAP.get(function) {
                        Some(f) => *f,
                        None => unreachable!("Function was checked to exist HAHHA"),
                    },
                    args,
                };
            })
            .collect()
    }
}

fn strip_quotes(s: &str) -> String {
    if s.starts_with('"') || s.starts_with('\'') {
        if s.len() >= 2 && s.chars().last() == s.chars().next() {
            return s[1..s.len() - 1].to_string();
        }
    }
    s.to_string()
}

fn split_args(input: &str) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_quotes = false;
    let mut quote_char = '\0';

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // Handle quotes
            '"' | '\'' => {
                if in_quotes && c == quote_char {
                    in_quotes = false;
                } else if !in_quotes {
                    in_quotes = true;
                    quote_char = c;
                }
                current.push(c);
            }

            // Handle parentheses nesting
            '(' if !in_quotes => {
                depth += 1;
                current.push(c);
            }
            ')' if !in_quotes => {
                if depth == 0 {
                    return Err("Unmatched closing parenthesis".into());
                }
                depth -= 1;
                current.push(c);
            }

            // Handle commas at top level
            ',' if !in_quotes && depth == 0 => {
                result.push(current.trim().to_string());
                current.clear();
            }

            // Default case
            _ => {
                current.push(c);
            }
        }
    }

    if in_quotes {
        return Err("Unclosed quote in arguments".into());
    }
    if depth != 0 {
        return Err("Unmatched opening parenthesis in arguments".into());
    }

    if !current.trim().is_empty() {
        result.push(current.trim().to_string());
    }

    Ok(result)
}

/// Converts given size as string to size in bytes 
fn parse_size(s: &str) -> Result<u64, String> {
    let s = s.trim().to_lowercase();

    let idx = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (num_part, suffix) = s.split_at(idx);

    let multiplier: i32 = match suffix.to_lowercase().as_str() {
        "b" => 1,
        "kb" => 1_000,
        "mb" => 1_000_000,
        "gb" => 1_000_000_000,
        "" => 1, //asumir byte (todo: option desde el user config)
        _ => {
            return Err("Invalid size suffix".into());
        }
    };

    let num: f64 = num_part
        .parse()
        .map_err(|_| "Invalid size number")?;

    Ok((num * (multiplier as f64)) as u64)
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%d/%m/%Y")
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .or_else(|_| NaiveDate::parse_from_str(s, "%m/%d/%Y"))
        .map_err(|_| format!("Could not parse date '{}'", s))
}

/// Arg Parsing
/// Literal
///     Any text sorounded by quotes or numbers
///     Ex. "some text", 123, 45.67
///
/// Path
///     A path, starts with "./" or "/". Using the " it's optional.
///     Ex. /some-path, ./../folder, "/a/b/c"
///
/// Conditional
///     Starts with a `Operator` and should be followed by a `Unit`
///     Ex. >100Mb, <6/12/2020
///
/// Group
///     Starts with "(", represents multiple Arguments separated by ","
///     Ex. ("abc", >100Ms)
///
fn parse_args(raw: &str) -> Result<Arg, String> {
    let raw = raw.trim();

    if raw.is_empty() {
        return Err(String::new());
    }

    let first_char = raw.chars().next().unwrap();

    // Group
    if first_char == '(' && raw.ends_with(')') {
        let inner = &raw[1..raw.len() - 1];
        let args = split_args(inner)?; // You’ll need to implement this separately
        let parsed: Result<Vec<Arg>, String> = args
            .iter()
            .map(|s| parse_args(s))
            .collect();
        return Ok(Arg::Group(parsed?));
    }

    // Conditional
    // (< | >){num}{unit}
    //       |   mm/dd/yyyy
    if first_char == '<' || first_char == '>' {
        let op = match first_char {
            '<' => Operator::Lt,
            '>' => Operator::Gt,
            _ => unreachable!(),
        };

        let raw_value = raw[1..].trim(); // strip the operator

        // Try parsing as size first
        if let Ok(size) = parse_size(raw_value) {
            return Ok(Arg::Conditional {
                operator: op,
                value: Unit::Size(size),
            });
        }

        // Try parsing as date
        if let Ok(date) = parse_date(raw_value) {
            return Ok(Arg::Conditional {
                operator: op,
                value: Unit::Date(date),
            });
        }

        return Err(format!("Could not parse conditional value: '{}'", raw_value));
    }

    // Path - must start with / or ./
    if
        raw.starts_with('/') ||
        raw.starts_with("./") ||
        raw.starts_with("\"/") ||
        raw.starts_with("\"./")
    {
        return Ok(Arg::Path(strip_quotes(raw)));
    }

    // Literal - quoted string
    if first_char == '"' || first_char == '\'' {
        if raw.len() < 2 || raw.chars().last() != Some(first_char) {
            return Err("Unclosed string".to_string());
        }

        return Ok(Arg::Literal(strip_quotes(raw)));
    }

    // Literal - number
    if raw.parse::<i32>().is_ok() || raw.parse::<f64>().is_ok() {
        return Ok(Arg::Literal(raw.to_string()));
    }

    // Fallback: treat as literal
    Ok(Arg::Literal(raw.to_string()))
}
