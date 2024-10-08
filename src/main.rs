use crate::configuration_and_utilities::*;
use crate::string_and_vec_parsing::*;
use crate::EvalTypes::EvalVec;
use convert_case::{Case, Casing};
use std::fmt::Debug;
use std::io;
use std::str::FromStr;
use strum_macros::EnumString;
mod configuration_and_utilities;
mod core_functions;
mod string_and_vec_parsing;

//Test string: x:[0,1,3.1419]:sin(x)*(cos(x)*(x+2*x^2))+sin(x)/(cos(x)*(x+2x^2))+2^x-4
//Test string: x:[0,1,2,3]:sin(x)/((1+x)^2)-(2+x)^2+1.1^x  OR  x:2:sin(x)/((1+x)^2)-(2+x)^2+1.1^x
//Answer should be [EvalVec([-3.0, -7.689632253798026, -14.688966952574923, -23.660179999496258])]
//Simple Test String: x:2:x*3-x+1   answer: 5
//Test string full: x:[4,6,2pi]:sin(x)*(cos(x)*(x+x))+sin(x)*(cos(x)*(x+x))+x^x-x
//Test string simple: x:[1,2]:(2+x*(x+1)+x+(x))/2
///Note: parens have to be around division blocks for accurate order of operations ie f(x)/g(x)
/// must be entered (f(x))/(g(x)). Exponentiation is treated with the same order of operations
/// as multiplication and division and all three will be evaluated from left to right.
/// If you have a potentially ambiguous statement, use parentheses such as x^(2*4) rather than
/// x^2*4.
fn main() {
    println!("Enter your equation: ");

    let mut use_input: String = String::new();
    io::stdin().read_line(&mut use_input).unwrap();
    use_input = use_input.trim().to_string();

    //Split inputs on colons, split_input handles the variable name, the input vector and the
    //expression to be evaluated.
    if use_input.to_lowercase() != "cancel"||use_input.to_lowercase()!="exit" {
        let split_input: Vec<&str> = use_input.split(':').collect();
        if split_input.len() != 3 {
            panic!(
            "Incorrect Formatting, input should be formatted as: x:2,4:x*3-x+1 or x:[2,4]:x*3-x+1"
        )
        }
        let var_name: &str = split_input[0];
        let input_vals: &str = split_input[1];
        let expression_string = split_input[2];

        let mut split_vals: Vec<&str> = vec!["(", ")", "*", "-", "+", "^", "/"];
        let x: Vec<&str> = input_vals.split(&['[', ']', ',']).collect();
        let input_vals: Vec<f64> = x
            .strip_empties()
            .iter()
            .map(|x| (*x).parse().unwrap())
            .collect();
        split_vals.push(var_name);

        let mut expression: Vec<String> =
            parse_on_parens(expression_string, split_vals, true, var_name);

        let solution = recursive_eval(&mut expression, &input_vals).extract_val();
        println!("{:?}", solution);
    } else {
        println!("Evaluation canceled.")
    }
}

///Eval types can store a StdFunction and a evaluation vector (just a Vec<f64>) variant. These
/// can be stored together in a vector to make flattened mathematical expressions.
#[derive(Clone, EnumString, Debug, PartialEq)]
pub enum EvalTypes {
    //to access stored val {if Some_placeholder(x)}
    None,
    EvalVec(Vec<f64>), //Holds vector of f64 for calculation by StdFunctions
    Fun(StdFunctions), //Holds member of StdFunctions which map to function handles
}

///Main call, recursively evaluates
pub fn recursive_eval(expression: &mut [String], input_vals: &Vec<f64>) -> EvalTypes {
    //Extract first and last elements of expression
    let exp_first_last = (
        expression.first().unwrap().to_string(),
        expression.last().unwrap().to_string(),
    );
    //Strip parentheses, if necessary, and set up a local variable for processing
    let loc_expression = {
        if exp_first_last == ("(".to_string(), ")".to_string()) {
            let exp_len = expression.len();
            expression[1..exp_len - 1].to_vec()
        } else {
            expression.to_vec()
        }
    };

    //Gather parentheses groups based on nesting. ((f(x))+g(x)) would create a bool containing the
    //indices of the first and last parentheses, and the nested parentheses will be handled on a
    //recursive call.
    let mut top_level_parens: Vec<(usize, usize)> = Vec::default();
    let mut working_ind_tuple: (usize, usize) = (0, 0);
    let mut paren_counter: (i32, i32) = (0, 0);
    let mut is_counting = false;
    let mut total_parens: i32 = 0;

    for (i, member) in loc_expression.clone().into_iter().enumerate() {
        if member == "(" {
            total_parens += 1;
            paren_counter.0 += 1;
            if !is_counting {
                working_ind_tuple.0 = i;
                is_counting = true;
            }
        } else if member == ")" {
            total_parens += 1;
            paren_counter.1 += 1;
            if paren_counter.0 == paren_counter.1 {
                is_counting = false;
                working_ind_tuple.1 = i;
                top_level_parens.push(working_ind_tuple);
                working_ind_tuple = (0, 0);
            }
        }
    }

    if total_parens % 2 == 1 {
        panic!("Parentheses mismatch, check your formatting!")
    }

    //Gets the indices for things outside of parentheses for later evaluation. Also sets mapping_vec,
    //which records the relationship between the nested parentheses groups and what is outside of them
    let (outside_parens, mapping_vec) =
        fill_indices(top_level_parens.clone(), loc_expression.len());

    let mut enums_vec_top: Vec<EvalTypes> = vec![EvalTypes::None; top_level_parens.len()]; // to store, nums_vec[i]=Some(x:f64)
    let mut enums_vec_outside: Vec<EvalTypes> = vec![EvalTypes::None; outside_parens.len()];

    //Recursive call, passes in nested parentheses slices of the evaluation expression. Stores final
    //output of nested parentheses calculation as a vector of Vec<f64> eval types.
    for (i, member) in top_level_parens.into_iter().enumerate() {
        if member.is_val_ind() {
            enums_vec_top[i] = recursive_eval(
                &mut loc_expression[member.0..=member.1].to_vec(),
                input_vals,
            );
        } else {
            panic!("Invalid Paren Configuration!")
        }
    }

    //Convert Strings at indices outside of parens into EvalTypes enum variants. Variable names
    //and numbers are converted into Vec<f64> and stored as EvalVec variants, and functions are stored
    //as Fun variant.
    for (i, member) in outside_parens.into_iter().enumerate() {
        //to fix need to iterate through each index BETWEEN member
        let match_var = loc_expression[member].as_str();
        if match_var.parse::<f64>().is_ok() {
            enums_vec_outside[i] =
                EvalVec(vec![match_var.parse::<f64>().unwrap(); input_vals.len()]);
        } else if match_var == "Variable" {
            enums_vec_outside[i] = EvalVec(input_vals.clone())
        } else {
            enums_vec_outside[i] = EvalTypes::Fun(
                StdFunctions::from_str(match_var.to_case(Case::UpperCamel).as_str()).unwrap(),
            )
        }
    }

    //Zips things that were inside parens with things that were outside parens based on the
    //mapping_vec. zipped_vector is a vector of EvalType variants which will be evaluated.
    let zipped_vector: Vec<EvalTypes> = zip_by_map(enums_vec_top, enums_vec_outside, mapping_vec);

    //Evaluate unary functions first. Note that eval_enum operates on vector in place, evaluating
    //the last two or three elements based on operation type and leaving the rest untouched
    let mut intermediate_vec_1: Vec<EvalTypes> = Vec::default();
    for member in zipped_vector {
        intermediate_vec_1.push(member.to_owned());
        intermediate_vec_1.eval_enum(UNARY_ENUMS.as_ref());
    }
    //Next in the order of operations do, from left to right, all operations involving multiplication
    let mut intermediate_vec_2: Vec<EvalTypes> = Vec::default();
    for member in intermediate_vec_1 {
        intermediate_vec_2.push(member.to_owned());
        intermediate_vec_2.eval_enum(BINARY_MULTIPLICATION_BASED_ENUMS.as_ref());
    }
    //Finally do addition and subtraction from left to right
    let mut soln_vec: Vec<EvalTypes> = Vec::default();
    for member in intermediate_vec_2 {
        soln_vec.push(member.to_owned());
        soln_vec.eval_enum(BINARY_ADDITION_BASED_ENUMS.as_ref());
    }

    if soln_vec.len() == 1 {
        soln_vec[0].clone()
    } else {
        panic!("Input was improperly formatted!")
    }
}

///Takes a Vec ind_tuples of tuples (usize,usize) and returns a vec<usize>. Returned vector contains the
///complement of the usize pairs in ind_tuples explicitly as a vector of usize. Additionally returns
/// a mapping vector of Strings which indicates how to interlace the complement vector and original
/// ind_tuples pairs to reproduce the structure of ind_tuples with all complement indexes filled in.
pub fn fill_indices(
    ind_tuples: Vec<(usize, usize)>,
    total_len: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut ind_tuples_iter = ind_tuples.iter().peekable();
    let mut working_vec: Vec<usize>;
    let mut output: Vec<usize> = Vec::default();
    let mut mapping_vector: Vec<String> = Vec::default();

    //Edge cases to fill in what comes before the first usize pair in ind_tuples, or fill in the whole
    //index set if the set of ind_tuples is empty.
    if ind_tuples.is_empty() {
        working_vec = (0usize..total_len).collect();
        output.append(&mut working_vec);
        mapping_vector.append(&mut vec!["use outside".to_string(); total_len]);
    } else if ind_tuples[0].0 > 0 {
        working_vec = (0usize..ind_tuples[0].0).collect();
        mapping_vector = vec!["use outside".to_string(); working_vec.len()];
        output = working_vec;
    }
    //iterate through ind_tuples, determine if the usize tuples leave space for a complement between them.
    //Concurrently construct a mapping vector to capture the relationships between the complement
    //and the usize tuples.
    while let Some(member) = ind_tuples_iter.next() {
        mapping_vector.push("use top".to_string());
        if let Some(member2) = ind_tuples_iter.peek() {
            if member.1 < { member2.0 - 1 } {
                working_vec = ((member.1 + 1)..member2.0).collect();
                let working_len = working_vec.len();
                output.append(&mut working_vec);
                mapping_vector.append(&mut vec!["use outside".to_string(); working_len])
            };
        }
    }
    //Handle anything not captured in the iteration, where the final tuple in ind_tuples does not cover
    //the entire Vec of calculations.
    if let Some(final_tuple) = ind_tuples.last() {
        if final_tuple.1 < (total_len - 1) {
            output.append(&mut (ind_tuples.last().unwrap().1 + 1usize..total_len).collect());
            mapping_vector.append(&mut vec![
                "use outside".to_string();
                (total_len)
                    - (ind_tuples.last().unwrap().1 + 1usize)
            ]);
        }
    }
    (output, mapping_vector)
}

///Implementation of enum evaluation for vectors of enum types.
impl EnumEval for Vec<EvalTypes> {
    ///Accepts a vector of eval types between length two and three. If the vector is of length 2,
    ///check if the operation is unary. Next, check if the vector is a valid binary operation.
    /// If neither, do not modify the Vec.
    fn eval_enum(&mut self, operations: &[StdFunctions]) {
        //Determine the length of vector slice to perform operation on
        let mut is_binary: bool = true;
        if UNARY_ENUMS.to_vec().contains(&operations[0]) {
            is_binary = false;
        }
        let check_width: usize = {
            if is_binary {
                3usize
            } else {
                2usize
            }
        };
        let mut check_self: Vec<EvalTypes> = Vec::default();
        let len_self = self.len();

        if len_self <= check_width {
            check_self = self.clone()
        } else if len_self > check_width {
            check_self = self.clone()[len_self - check_width..len_self].to_vec()
        };

        if is_binary && check_self.len() == 3 {
            if operations.contains(&check_self[1].to_std_fn()) {
                let current_fn: Binop =
                    BINARY_HANDLES.to_vec()[(check_self.clone()[1].to_std_fn().to_f64()
                        + (StdFunctions::NumberOfBinaryFns as i32 as f64))
                        as usize];
                check_self = vec![EvalVec(current_fn(
                    check_self[0].extract_val(),
                    check_self[2].extract_val(),
                ))]
            }
        } else if !is_binary
            && check_self.len() == 2
            && operations.contains(&check_self[0].to_std_fn())
        {
            let current_fn: Unop = UNARY_HANDLES.to_vec()
                [check_self.clone()[0].to_std_fn().to_f64() as usize - 1usize];
            //^This converts the discriminant of a StdFunction variant to a function handle based on
            //the list of unary function handles above.
            check_self = vec![EvalVec(current_fn(check_self[1].extract_val()))];
        }
        if len_self <= check_width {
            *self = check_self
        } else {
            let mut holder = self.clone()[0usize..len_self - check_width].to_vec();
            holder.append(&mut check_self);
            *self = holder;
        }
    }
}

///trait to implement checking if a tuple (usize,usize) defines a valid parentheses pair
trait PairBool {
    fn is_val_ind(&self) -> bool;
}

///Trait to evaluate a vector of EvalTypes
trait EnumEval {
    fn eval_enum(&mut self, operations: &[StdFunctions]);
}

///implementation of pair_bool to check if a tuple (usize,usize) defines a valid parentheses
impl PairBool for (usize, usize) {
    ///Check if a tuple of B=(usize,usize) defines a valid parentheses pair.
    fn is_val_ind(&self) -> bool {
        (self.1 as i32) - (self.0 as i32) > 0
    }
}

///Converts an EvalType to either its discriminant in StdFunctions, or NAN if it is not of the valid
/// variant. NAN is returned to make if statements with inequalities automatically false. This
/// discriminant value is not used in any calculations, so there is no risk of NAN propagation.
impl StdFunctions {
    fn to_f64(self) -> f64 {
        match self {
            StdFunctions::None => f64::NAN,
            _ => self as i32 as f64,
        }
    }
}

impl EvalTypes {
    ///Extract the value of the EvalVec variant of EvalTypes. If a functional type is passed, code
    /// calls a panic as this implies the input was incorrectly formatted.
    fn extract_val(&self) -> Vec<f64> {
        let EvalVec(out_vec) = &self else {
            panic!("Wrong Eval Type! (In extract_val)")
        };
        out_vec.clone()
    }
    ///Extract the StdFunction variant from the SomeEnum variant of EvalTypes. If the type is
    /// inappropriate, return the StdFunctions::None variant.
    fn to_std_fn(&self) -> StdFunctions {
        if let EvalTypes::Fun(guy) = &self {
            *guy
        } else {
            StdFunctions::None
        }
    }
}

///Debugging print utility to save on typing. Also serves as generics practice.
pub fn dbg_print<T: Debug>(x: T) {
    println!("{:?}", x);
}
