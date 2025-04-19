use chrono::NaiveDate;



pub type FunctionTy = fn(&str, &Arg) -> Result<bool, String>; 


#[derive(Debug, Clone)]
pub enum Operator {
    Lt,
    Gt,
}

#[derive(Debug, Clone)]
pub enum Unit {
    Date(NaiveDate),
    Size(u64) 
}

#[derive(Debug, Clone)]
pub enum Arg {
    Literal(String),  // Strings or numbers

    Path(String), // I'm assuming Path's will have a different treatment than Literals, but this could later be a literal

    // Represent a condion such as modified:>2024-01-01 
    // In this example: operator=">", value=Unit::Date("2024-01-01")
    Conditional {
        operator: Operator,
        value: Unit,
    },

    Group(Vec<Arg>)
}


#[derive(Debug, Clone)]
pub enum Node {
    Fail(String),

    /// A sucessful parsed `function:args` item
    Call {
        func: FunctionTy, // Function to call,
        args: Arg 
    }
} 