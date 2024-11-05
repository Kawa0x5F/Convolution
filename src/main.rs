use std::io::stdin;

fn main() {
    let x1 = read_vec_float32();
    let x2 = read_vec_float32();

    let mut ans: Vec<f32> = vec![0_f32; x1.len() + x2.len() - 1];

    for i in 0 .. x1.len() { 
        for j in 0 .. x2.len() {
            ans[i + j] += x1[i] * x2[j];
        }
    }

    println!("{:?}", ans);
}

/**
 * 標準入力を受け取り，それをfloat32bit型のVectorとして返す
 */
fn read_vec_float32() -> Vec<f32> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    return line.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}