///To introduce a new operation, you need to add it to the handles and enums statics, the
/// StdFunctions enum, and the string to enum parser.
use strum_macros::EnumString;
use crate::core_functions::*;

///These functions take only one input and will be evaluated first, treated as a parentheses group
pub static UNARY_HANDLES: [Unop;2] = [sin_custom,cos_custom];
pub static UNARY_ENUMS: [StdFunctions;2]=[StdFunctions::Sin,StdFunctions::Cos];

///These operations involve multiplication and will be applied first in the order of operations
pub static BINARY_MULTIPLICATION_BASED_ENUMS: [StdFunctions;3]=[StdFunctions::Divide,StdFunctions::Multiply,StdFunctions::Power];
pub static BINARY_ADDITION_BASED_ENUMS: [StdFunctions;2]=[StdFunctions::Add,StdFunctions::Subtract];
///These operations involve addition and will be applied second in the order of operations
pub static BINARY_HANDLES: [Binop;5]=[add,subtract_custom,multiply,divide,power];

pub type Binop =fn(Vec<f64>, Vec<f64>) ->Vec<f64>;
pub type Unop =fn(Vec<f64>) -> Vec<f64>;

///Contains function names whose discriminants can be mapped to the function handles contained
/// in the statics in configuration_and_utilities
#[derive(EnumString, Clone, Copy, Debug, Default, PartialEq)]
pub enum StdFunctions {
    // Negative discriminant are binary functions, positive are unary functions
    #[default]
    None, //needs to be 0
    Add = -5,
    Subtract = -4,
    Multiply = -3, //Binary operations are negative, add NumberOfBinaryFunctions to map to handles
    Divide = -2,
    Power = -1,
    NumberOfBinaryFns = 5, //Note that this discriminator is reserved, and has to be referred to a
    Sin = 1,               //null function in the list of functions. If I make too many unary
    Cos = 2,               //functions, they can be shifted in index to go from NumberOfBinaryFns
    Variable = 10,         //to the upper limit.
    //Mathematical constants, these can be implemented later. Enum discriminant can be made large
    //to keep them out of the way if necessary.
    E = 11,  //mathematical constant e
    Pi = 12, //mathematical constant pi
}

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