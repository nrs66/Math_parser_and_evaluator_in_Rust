use crate::configuration_and_utilities::convert_string_to_enum;
use crate::EvalTypes;

///Parses a string by splitting on any input list and returning the members of the input list
/// as part of the string. If do_enums is true then replace operations with their enum names
/// and var_name with "Variable".
pub fn parse_on_parens(
    input_string: &str,
    char_list: Vec<&str>,
    do_enums: bool,
    var_name: &str,
) -> Vec<String> {
    let mut working_word: String = String::new();
    let mut output_vec: Vec<String> = vec![];

    for c in input_string.chars() {
        if char_list.contains(&&c.to_string().as_str()) {
            if !working_word.is_empty() {
                output_vec.push(
                    convert_string_to_enum(working_word.as_str(), do_enums, var_name.to_string())
                        .to_string(),
                );
                working_word = String::new();
            }
            output_vec.push(
                convert_string_to_enum(&c.to_string(), do_enums, var_name.to_string()).to_string(),
            )
        } else {
            working_word.push_str(c.to_string().as_str())
        }
    }

    if !working_word.is_empty() {
        output_vec.push(
            convert_string_to_enum(working_word.as_str(), do_enums, var_name.to_string())
                .to_string(),
        )
    }
    //output_vec.push(working_word);
    output_vec
}

///Trait to strip empty strings from a vector of string slices
pub trait CanStripEmpties {
    fn strip_empties(&self) -> Vec<&str>;
}

///Removes empty &str from a vector of &str
impl CanStripEmpties for Vec<&str> {
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

///Function to take two vectors and zip them based on a mapping vector. Mapping vector must have format
/// using Strings "use top" and "use outside". May later generalize this notation and use a Vec of
/// binary values to make this utility function usable in more generic settings.
pub fn zip_by_map(
    topside: Vec<EvalTypes>,
    outside: Vec<EvalTypes>,
    mapping_vec: Vec<String>,
) -> Vec<EvalTypes> {
    let mut final_vec: Vec<EvalTypes> = Vec::default();
    //let total_len=topside.len()+outside.clone().len();
    let mut outside_ind: usize = 0usize;
    let mut topside_ind: usize = 0usize;
    let mut vec_slice: EvalTypes;

    for mapper in mapping_vec.iter() {
        match mapper as &str {
            "use outside" => {
                vec_slice = (&outside[outside_ind]).clone();
                final_vec.push(vec_slice);
                outside_ind += 1;
            }
            "use top" => {
                vec_slice = topside[topside_ind].clone();
                final_vec.push(vec_slice);
                topside_ind += 1;
            }
            _ => {
                panic!("Something's wrong at the zipping stage.")
            }
        }
    }

    final_vec
}