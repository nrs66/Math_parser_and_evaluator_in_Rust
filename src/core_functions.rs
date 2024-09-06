pub fn add(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::default();
    for (i,member1) in input1.iter().enumerate(){
        out_vec.push(member1+input2[i]);
    }
    out_vec
}
pub fn subtract_custom(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::default();
    for (i,member1) in input1.iter().enumerate(){
        out_vec.push(member1-input2[i]);
    }
    out_vec
}
pub fn multiply(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::default();
    for (i,member1) in input1.iter().enumerate(){
        out_vec.push(member1*input2[i]);
    }
    out_vec
}
pub fn divide(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::default();
    for (i,member1) in input1.iter().enumerate(){
        out_vec.push(member1/input2[i]);
    }
    out_vec
}
pub fn power(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::default();
    for (i,member1) in input1.iter().enumerate(){
        out_vec.push(member1.powf(input2[i]));
    }
    out_vec
}
pub fn sin_custom(input: Vec<f64>) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::default();
    for member in input.iter() {
        out_vec.push(member.sin());
    }
    out_vec
}
pub fn cos_custom(input: Vec<f64>) -> Vec<f64> {
    let mut out_vec: Vec<f64> = Vec::default();
    for member in input.iter() {
        out_vec.push(member.cos());
    }
    out_vec
}
