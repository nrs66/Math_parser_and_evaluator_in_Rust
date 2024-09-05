pub fn Add(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    println!("Function inputs: {:?}, {:?}", input1, input2);
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] + input2[i]);
    }
    out_vec
}
pub fn Subtract_Nick(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] - input2[i]);
    }
    out_vec
}
pub fn Multiply(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] * input2[i]);
    }
    out_vec
}
pub fn Divide(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i] / input2[i]);
    }
    out_vec
}
pub fn Power(input1: Vec<f64>, input2: Vec<f64>) -> Vec<f64> {
    let vec_len = input1.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input1[i].powf(input2[i]));
    }
    out_vec
}
pub fn Sin_Nick(input: Vec<f64>) -> Vec<f64> {
    let vec_len = input.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input[i].sin());
    }
    out_vec
}
pub fn Cos_Nick(input: Vec<f64>) -> Vec<f64> {
    let vec_len = input.len();
    let mut out_vec: Vec<f64> = Vec::default();
    for i in 0..vec_len {
        out_vec.push(input[i].cos());
    }
    out_vec
}
