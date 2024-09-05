pub fn add(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] + input2[i]);
    }
    out_vec
}
pub fn subtract_custom(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] - input2[i]);
    }
    out_vec
}
pub fn multiply(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] * input2[i]);
    }
    out_vec
}
pub fn divide(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] / input2[i]);
    }
    out_vec
}
pub fn power(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i].powf(input2[i]));
    }
    out_vec
}
pub fn sin_custom(input: Vec<f64>) -> Vec<f64> {
    let vec_len = input.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input[i].sin());
    }
    out_vec
}
pub fn cos_custom(input: Vec<f64>) -> Vec<f64> {
    let vec_len = input.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input[i].cos());
    }
    out_vec
}
