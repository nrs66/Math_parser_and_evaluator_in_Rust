use crate::CoreFunctions::*;
use crate::EvalTypes::EvalVec;
use convert_case::{Case, Casing};
use itertools::*;
use std::any::Any;
use std::fmt::Debug;
use std::io;
use std::str::FromStr;
use strum_macros::EnumString;

mod CoreFunctions;

//Test string: x:[0,1,3.1419]:sin(x)*(cos(x)*(x+2*x^2))+sin(x)/(cos(x)*(x+2x^2))+2^x-4
//Test string: x:[0,1,2,3]:sin(x)/((1+x)^2)-(2+x)^2+1.1^x
//Answer should be [EvalVec([-3.0, -7.689632253798026, -14.688966952574923, -23.660179999496258])]
//Test string full: x:[4,6,2pi]:sin(x)*(cos(x)*(x+x))+sin(x)*(cos(x)*(x+x))+x^x-x
//Test string simple: x:[1,2]:(2+x*(x+1)+x+(x))/2
///Note: parens have to be around division blocks for accurate order of operations ie f(x)/g(x)
/// must be entered (f(x))/(g(x))
fn main() {
    println!("Enter your equation: ");
    // input as x:[2,3,4]:sin{x} or varname:sin{varname}

    let f: StdFunctions = StdFunctions::Add;
    //f.eval_enum(4.5);

    let mut use_input: String = String::new();
    io::stdin().read_line(&mut use_input).unwrap();
    use_input = use_input.trim().to_string();

    let split_input: Vec<&str> = use_input.split(':').collect();
    if split_input.len() != 3 {
        panic!("Incorrect Formatting")
    }

    let varname: &str = split_input[0];
    let input_vals: &str = split_input[1];
    //let mut expression : Vec<&str>=split_input[2].split_inclusive(&['(', ')'][..]).collect();
    let mut split_vals: Vec<&str> = vec!["(", ")", "*", "-", "+", "^", "/"];
    let x: Vec<&str> = input_vals.split(&['[', ']', ',']).collect();
    let input_vals: Vec<f64> = x
        .strip_empties()
        .iter()
        .map(|x| (*x).parse().unwrap())
        .collect();
    split_vals.push(varname);
    let pass_string = split_input[2];
    let mut expression: Vec<String> = parse_on_parens(pass_string, split_vals, true, varname);

    recursive_eval(&mut expression, &(varname.to_string()), &input_vals);
}

#[derive(EnumString, Clone, Copy, Debug, Default, PartialEq)]
pub enum StdFunctions {
    // Negative descriminators are binary functions, positive are unitary functions
    #[default]
    None, //needs to be 0
    Add = -5,
    Subtract = -4,
    Multiply = -3, //Note this is going to need to be re-ordered in the function list!
    Divide = -2,
    Power = -1,
    NumberOfBinaryFuncs = 5, //Note that this discriminator is reserved, and has to be referred to a
    Sin = 1,                 //null function in the list of functions. If I make too many unitary
    Cos = 2,                 //functions, they can be shifted in index to go from NumberofBinaryFuncs
    Variable = 10,           //to the upper limit.
    //Mathematical constants, these can be implemented later. Enum discriminiator can be made large
    //to keep them out of the way if necessary.
    E = 11, //mathematical constant e
    Pi = 12, //mathematical constant pi
}

#[derive(Clone, EnumString, Debug, PartialEq)]
pub enum EvalTypes {
    //to access stored val {if Some_placeholder(x)}
    None, //Can I hold function handles in enums? Yes!
    EvalVec(Vec<f64>),
    Const(f64),
    SomeEnum(StdFunctions),
}

//Test string full: x:[4,6,2pi]:sin(x)(cos(x)(x+2x^2))+sin(x)(cos(x)(x+2x^2))+e^x-4
pub fn recursive_eval(
    expression: &mut Vec<String>,
    varname: &String,
    input_vals: &Vec<f64>,
) -> EvalTypes {
    //Extract Parentheses Groups
    println!("Run recursive evaluation.");
    //Change this to stripping parens first
    //let mut loc_expression:Vec<String> =Vec::new();
    let exp_first_last = (
        expression.first().unwrap().to_string(),
        expression.last().unwrap().to_string(),
    );
    let mut loc_expression = {
        if (exp_first_last.0 == "(") && (exp_first_last.1 == ")") {
            let explen = expression.len();
            expression[1..explen - 1].to_vec()
        } else {
            expression.to_vec()
        }
    };

    println!("Recursion operating on {:?}", loc_expression);

    //Gather parentheses groups based on logical imputation. ((f(x))+g(x)) returns first and last index
    let mut top_level_parens: Vec<(usize, usize)> = Vec::default();
    let mut working_inds: (usize, usize) = (0, 0);
    let mut paren_counter: (i32, i32) = (0, 0);
    let mut is_counting = false;

    for (i, member) in loc_expression.clone().into_iter().enumerate() {
        if member == "(" {
            paren_counter.0 += 1;
            if is_counting == false {
                working_inds.0 = i;
                is_counting = true;
            }
        } else if member == ")" {
            paren_counter.1 += 1;
            if paren_counter.0 == paren_counter.1 {
                is_counting = false;
                working_inds.1 = i;
                top_level_parens.push(working_inds);
                working_inds = (0, 0);
            }
        }
    }

    println!("Top level parens: {:?}", top_level_parens);

    let (outside_parens, mapping_vec) = fill_inds(top_level_parens.clone(), loc_expression.len());

    let mut enums_vec_top: Vec<EvalTypes> = vec![EvalTypes::None; top_level_parens.len()]; // to store, nums_vec[i]=Some(x:f64)
    let mut enums_vec_outside: Vec<EvalTypes> = vec![EvalTypes::None; outside_parens.len()];
    let ops_vec: Vec<String> = vec![String::new(); top_level_parens.len()];

    for (i, member) in top_level_parens.into_iter().enumerate() {
        if member.is_val_ind()
        //&&!was_empty
        {
            println!(
                "Sent to recursion: {:?}",
                loc_expression[member.0..=member.1].to_vec()
            );
            enums_vec_top[i] = recursive_eval(
                &mut loc_expression[member.0..=member.1].to_vec(),
                varname,
                input_vals,
            );
        } else {
            panic!("Invalid Paren Configuration!")
        }
    }

    for (i, member) in outside_parens.into_iter().enumerate() {
        //to fix need to iterate through each index BETWEEN member
        let match_var = (&loc_expression[member]).as_str();
        if match_var.parse::<f64>().is_ok() {
            enums_vec_outside[i] =
                EvalTypes::EvalVec(vec![match_var.parse::<f64>().unwrap(); input_vals.len()]);
        } else if match_var == "Variable" {
            enums_vec_outside[i] = EvalVec(input_vals.clone())
        } else {
            enums_vec_outside[i] = EvalTypes::SomeEnum(
                StdFunctions::from_str(match_var.to_case(Case::UpperCamel).as_str()).unwrap(),
            )
        }
    }

    println!("Enums inside parens: {:?}", enums_vec_top);
    println!("Enums outside parens: {:?}", enums_vec_outside);
    println!("Mapping Vector: {:?}", mapping_vec);

    let zipped_vector: Vec<EvalTypes> = NicksZipper(enums_vec_top, enums_vec_outside, mapping_vec);

    println!("Zipped vector: {:?}", zipped_vector);

    let mut working_vec: Vec<EvalTypes> = Vec::default();
    let mut soln_vec: Vec<EvalTypes> = Vec::default();

    let first_ops = vec![StdFunctions::Sin, StdFunctions::Cos];

    //Consider using global variables to handle things concerning categories of functions. Look at
    //static variables
    let first_ops = vec![StdFunctions::Sin, StdFunctions::Cos];
    let second_ops = vec![
        StdFunctions::Multiply,
        StdFunctions::Divide,
        StdFunctions::Power,
    ];

    let mut intermediate_vec: Vec<EvalTypes> =
        eval_type_of_operator(zipped_vector, first_ops, false);
    intermediate_vec = eval_type_of_operator(intermediate_vec, second_ops, true);

    let mut inter_iter = intermediate_vec.iter();
    soln_vec.push(inter_iter.next().unwrap().to_owned());

    while let Some(member) = inter_iter.next() {
        println!("{:?}", working_vec);
        soln_vec.push(member.to_owned());
        soln_vec.eval_enum();
    }

    println!("Answer: {:?}", soln_vec);

    (soln_vec)[0].clone()
}

///Iterate through the indices Vec<EvalTypes> and perform the list of operations. The bool isbinary
/// determines if we are working with a binary(isbinary=true) or unitary(isbinary=false) operation.
/// Returns a Vec<EvalTypes> with these operations completed.
pub fn eval_type_of_operator(
    eval_vec: Vec<EvalTypes>,
    operations: Vec<StdFunctions>,
    isbinary: bool,
) -> Vec<EvalTypes> {
    //working_vec contains the current proposed calculation. intermediate_vec contains output.
    let mut working_vec: Vec<EvalTypes> = Vec::default();
    let mut intermediate_vec: Vec<EvalTypes> = Vec::default();
    let eval_len = eval_vec.len();
    let mut ind_iter = (0usize..eval_len).into_iter();

    if isbinary == true {
        //If possible, construct an evaluation vector
        while let Some(i) = ind_iter.next() {
            if let Some(next_guy) = eval_vec.get(i + 1).to_owned() {
                if let Some(nnext_guy) = eval_vec.get(i + 2).to_owned() {
                    working_vec = vec![eval_vec[i].clone(), next_guy.clone(), nnext_guy.clone()];
                    println!("Working Vec is {:?}", working_vec);
                    //Check if the second element of the working_vec is a standard function, and
                    //if it is in the list of functions we wish to evaluate. If it is not a function
                    //enum variant, convert_to_StdFn returns StdFunctions::None.
                    if operations.contains(&working_vec[1].convert_to_StdFn()) {
                        working_vec.eval_enum();
                        println!("working vec after eval: {working_vec:?}");
                        intermediate_vec.append(&mut working_vec);
                        //If evaluation is done, skip the next two positions as they have been
                        //consumed by the operation.
                        ind_iter.next();
                        ind_iter.next();
                    } else {
                        intermediate_vec.push(eval_vec[i].to_owned());
                    }
                } else {
                    intermediate_vec.push(eval_vec[i].to_owned());
                }
            } else {
                intermediate_vec.push(eval_vec[i].to_owned());
            }
            println!("Intermediate Vec: {:?}", intermediate_vec);
        }
    } else {
        while let Some(i) = ind_iter.next() {
            if let Some(next_guy) = eval_vec.get(i + 1).to_owned() {
                working_vec = vec![eval_vec[i].clone(), next_guy.clone()];
                println!("Working Vec is {:?}", working_vec);
                if operations.contains(&working_vec[0].convert_to_StdFn()) {
                    working_vec.eval_enum();
                    println!("working vec after eval: {working_vec:?}");
                    intermediate_vec.append(&mut working_vec);
                    ind_iter.next();
                } else {
                    intermediate_vec.push(eval_vec[i].to_owned());
                }
            } else {
                intermediate_vec.push(eval_vec[i].to_owned());
            }
            println!("Intermediate Vec: {:?}", intermediate_vec);
        }
    }
    intermediate_vec
}

///Parses a string by splitting on any input list and returning the members of the input list
/// as part of the string. If do_enums is true then replace operations with their enum names
/// and varname with "Variable".
pub fn parse_on_parens(
    input_string: &str,
    char_list: Vec<&str>,
    do_enums: bool,
    varname: &str,
) -> Vec<String> {
    let mut working_word: String = String::new();
    let mut output_vec: Vec<String> = vec![];

    for c in input_string.chars() {
        if char_list.contains(&&c.to_string().as_str()) {
            if !working_word.is_empty() {
                output_vec.push(
                    convert_string_to_enum(working_word.as_str(), do_enums, varname.to_string())
                        .to_string(),
                );
                working_word = String::new();
            }
            output_vec.push(
                convert_string_to_enum(&c.to_string(), do_enums, varname.to_string()).to_string(),
            )
        } else {
            working_word.push_str(c.to_string().as_str())
        }
    }

    if !working_word.is_empty() {
        output_vec.push(
            convert_string_to_enum(working_word.as_str(), do_enums, varname.to_string())
                .to_string(),
        )
    }
    //output_vec.push(working_word);
    output_vec
}

///Takes &str and, if relevant, converts it to a valid StdFunctions enum variant name. This permits
/// us to use strum utilities to extract enum variants from the string slices.
pub fn convert_string_to_enum(input: &str, check: bool, varname: String) -> &str {
    if check {
        match input {
            "+" => "Add",
            "/" => "Divide",
            "-" => "Subtract",
            "^" => "Power",
            "*" => "Multiply",
            "sin" => "Sin",
            "cos" => "Cos",
            x if x == varname => "Variable",
            _ => input,
        }
    } else {
        input
    }
}

///Takes a Vec inds of tuples (usize,usize) and returns a vec<usize>. Returned vector contains the
///complement of the usize pairs in inds explicitly as a vector of usize. Additionally returns
/// a mapping vector of Strings which indicates how to interlace the complement vector and original
/// inds pairs to reproduce the structure of inds with all complemement indexes filled in.
pub fn fill_inds(inds: Vec<(usize, usize)>, total_len: usize) -> (Vec<usize>, Vec<String>) {
    let mut inds_iter = inds.iter().peekable();
    let mut working_vec: Vec<usize> = Vec::default();
    let mut output: Vec<usize> = Vec::default();
    let mut mapping_vector: Vec<String> = Vec::default();

    //Edge cases to fill in what comes before the first usize pair in inds, or fill in the whole
    //index set if the set of inds is empty.
    if inds.is_empty() {
        working_vec = (0usize..total_len).collect();
        output.append(&mut working_vec);
        mapping_vector.append(&mut vec!["use outside".to_string(); total_len]);
    } else if inds[0].0 > 0 {
        working_vec = (0usize..inds[0].0).collect();
        mapping_vector = vec!["use outside".to_string(); working_vec.len()];
        output = working_vec;
    }
    //iterate through inds, determine if the usize tuples leave space for a complement between them.
    //Concurrently construct a mapping vector to capture the relationships between the complement
    //and the usize tuples.
    while let Some(member) = inds_iter.next() {
        //output.push(*member);
        mapping_vector.push("use top".to_string());
        if let Some(member2) = inds_iter.peek() {
            if member.1 < { (*member2).0 - 1 } {
                working_vec = ((member.1 + 1)..member2.0).collect();
                let working_len = working_vec.len();
                output.append(&mut working_vec);
                mapping_vector.append(&mut vec!["use outside".to_string(); working_len])
            };
        }
    }
    //Handle anything not captured in the iteration, where the final tuple in inds does not cover
    //the entire Vec of calculations.
    if let Some(Guy) = inds.last() {
        if Guy.1 < (total_len - 1) {
            output.append(&mut (inds.last().unwrap().1 + 1usize..total_len).collect());
            mapping_vector.append(&mut vec![
                "use outside".to_string();
                (total_len) - (inds.last().unwrap().1 + 1usize)
            ]);
        }
    }

    (output, mapping_vector)
}

///Function to take two Vec's and zip them based on a mapping vector. Mapping vector must have format
/// using Strings "use top" and "use outside". May later generalize this notation and use a Vec of
/// binary values to make this utility function usable in more generic settings.
fn NicksZipper(
    topside: Vec<EvalTypes>,
    outside: Vec<EvalTypes>,
    mapping_vec: Vec<String>,
) -> Vec<EvalTypes> {
    let mut final_vec: Vec<EvalTypes> = Vec::default();
    //let total_len=topside.len()+outside.clone().len();
    let mut outside_ind: usize = 0usize;
    let mut topside_ind: usize = 0usize;
    let mut vecslice: EvalTypes;

    for mapper in mapping_vec.iter() {
        match mapper as &str {
            "use outside" => {
                vecslice = (&outside[outside_ind]).clone();
                final_vec.push(vecslice);
                outside_ind += 1;
            }
            "use top" => {
                vecslice = (&topside[topside_ind]).clone();
                final_vec.push(vecslice);
                topside_ind += 1;
            }
            _ => {
                panic!("Something's wrong at the zipping stage.")
            }
        }
    }

    final_vec
}

///trait to implement checking if a tuple (usize,usize) defines a valid parentheses pair
trait pair_bool {
    fn is_val_ind(&self) -> bool;
}

///Trait to evaluate a vector of EvalTypes
trait enum_eval {
    fn eval_enum(&mut self);
}

///Trait to strip empty strings from a vector of string slices
trait can_strip_empties {
    fn strip_empties(&self) -> Vec<&str>;
}

///Removes empty &str from a vector of &str
impl can_strip_empties for Vec<&str> {
    fn strip_empties(&self) -> Vec<&str> {
        let mut output: Vec<&str> = Vec::default();
        for s in self.into_iter() {
            if s.is_empty() {
            } else {
                output.push(s)
            };
        }
        output
    }
}

///implementation of pair_bool to check if a boolean of usizes defines a valid parentheses
impl pair_bool for (usize, usize) {
    ///Check if a tuple of B=(usize,usize) defines a valid parentheses pair.
    fn is_val_ind(&self) -> bool {
        (*&self.1 as i32) - (*&self.0 as i32) > 0
    }
}

///Converts an EvalType to either its discriminant in StdFunctions, or NAN if it is not of the valid
/// variant. NAN is returned to make if statements with inequalities automatically false. This
/// discriminant value is not used in any calculations, so there is no risk of NAN propagation.
impl StdFunctions {
    fn to_f64_Nick(&self) -> f64 {
        match &self {
            StdFunctions::None => f64::NAN,
            _ => *self as i32 as f64,
        }
    }
}

///Implementation of enum evaluation for vectors of enum types.
impl enum_eval for Vec<EvalTypes> {
    ///Accepts a vector of eval types between length two and three. If the vector is of length 2,
    ///check if the operation is unitary. Next, check if the vector is a valid binary operation.
    /// If neither, do not modify the Vec.
    fn eval_enum(&mut self) {
        let check_self = &self;
        //Declare all available function names in the order determined by the descriminants of
        //StdFunctions. Note that I would like to move this information into a global variable
        //which is generated on runtime to reduce the number of locations where this information
        //must be contained and modified if additional functions are added. Global variables would be
        //a vector of function handles, and a vector of associated enums.
        let avail_bin_fns: Vec<fn(Vec<f64>, Vec<f64>) -> Vec<f64>> =
            vec![Add, Subtract_Nick, Multiply, Divide, Power];
        let avail_un_fns: Vec<fn(Vec<f64>) -> Vec<f64>> = vec![Sin_Nick, Cos_Nick];
        //In the case of a unitary operation, first element should be the SomeEnum variant and the
        //second should be the EvalTypes variant. If the input string was incorrectly formatted
        //there will be a panic here from an inappropriate conversion. Note that if the first element
        //is not a function, convert)to_StdFn will return NAN rendering the inequality false.
        //Subsequent case works the same way for binary functions with appropriate changes.
        if (check_self[0].convert_to_StdFn().to_f64_Nick()) > 0f64 {
            type my_func = fn(Vec<f64>) -> Vec<f64>;
            let current_fn: my_func =
                avail_un_fns[self.clone()[0].convert_to_StdFn().to_f64_Nick() as usize - 1usize];
            //^This converts the discriminant of a StdFunction variant to a function handle based on
            //the list of unitary function handles above.
            *self = vec![EvalVec(current_fn(self[1].extract_val()))]
        } else if (check_self[1].convert_to_StdFn().to_f64_Nick() < 0f64) && (check_self.len() > 2)
        {
            type my_func = fn(Vec<f64>, Vec<f64>) -> Vec<f64>;
            let current_fn: my_func =
                avail_bin_fns[(self.clone()[1].convert_to_StdFn().to_f64_Nick()
                    + (StdFunctions::NumberOfBinaryFuncs as i32 as f64))
                    as usize];
            *self = vec![EvalVec(current_fn(
                self[0].extract_val(),
                self[2].extract_val(),
            ))]
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
    fn convert_to_StdFn(&self) -> StdFunctions {
        println!("Check function convert: {:?}", self);
        if let EvalTypes::SomeEnum(guy) = &self {
            *guy
        } else {
            StdFunctions::None
        }
    }
}

///Debugging print utility to save on typing. Also serves as generics practice.
pub fn dbgprnt<T: Debug>(x: T) {
    println!("{:?}", x);
}



