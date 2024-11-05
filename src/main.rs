use std::io::stdin;

fn main() {
    let x1 = read_vec_float32();
    let x2 = read_vec_float32();

    println!("{:?}", x1);
    println!("{:?}", x2);
}

/**
 * 標準入力を受け取り，それをfloat32bit型のVectorとして返す
 */
fn read_vec_float32() -> Vec<f32> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    return line.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}