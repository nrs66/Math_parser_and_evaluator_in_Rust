use crate::core_functions::*;
use crate::StdFunctions;

///These functions take only one input and will be evaluated first, treated as a parentheses group
pub static UNARY_HANDLES: [fn(Vec<f64>) -> Vec<f64>;2] = [sin_custom,cos_custom];
pub static UNARY_ENUMS: [StdFunctions;2]=[StdFunctions::Sin,StdFunctions::Cos];

///These operations involve multiplication and will be applied first in the order of operations
pub static BINARY_FIRST_HANDLES: [fn(Vec<f64>, Vec<f64>) -> Vec<f64>;3] = [multiply, divide, power];
pub static BINARY_FIRST_ENUMS: [StdFunctions;3]=[StdFunctions::Divide,StdFunctions::Multiply,StdFunctions::Power];

///These operations involve addition and will be applied second in the order of operations
pub static BINARY_SECOND_HANDLES: [fn(Vec<f64>, Vec<f64>) -> Vec<f64>;2] = [add, subtract_custom];
pub static BINARY_SECOND_ENUMS: [StdFunctions;2]=[StdFunctions::Add,StdFunctions::Subtract];

///Takes &str and, if relevant, converts it to a valid StdFunctions enum variant name. This permits
/// us to use strum utilities to extract enum variants from the string slices.
pub fn convert_string_to_enum(input: &str, check: bool, var_name: String) -> &str {
    if check {
        match input {
            "+" => "Add",
            "/" => "Divide",
            "-" => "Subtract",
            "^" => "Power",
            "*" => "Multiply",
            "sin" => "Sin",
            "cos" => "Cos",
            x if x == var_name => "Variable",
            _ => input,
        }
    } else {
        input
    }
}