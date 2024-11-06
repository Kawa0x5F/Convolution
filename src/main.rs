use std::io::stdin; // 標準入力用のモジュール

fn main() {
    let x1 = read_vec_float32(); // 任意の浮動小数の値で構成されるVector型で不変変数 x1 を束縛
    let x2 = read_vec_float32(); // 任意の浮動小数の値で構成されるVector型で不変変数 x2 を束縛

    // 可変変数 ans を ( x1の要素数 + x2の要素数 - 1 ) の数分 0 で初期化された f32型のVector型で束縛
    let mut ans: Vec<f32> = vec![0_f32; x1.len() + x2.len() - 1];

    // 畳み込みの計算
    for i in 0..x1.len() {
        // x1の長さだけループする
        for j in 0..x2.len() {
            // x2の長さだけループする
            // 答えを格納する場所をずらしながら加算していく
            ans[i + j] += x1[i] * x2[j];
        }
    }

    println!("Convolution result : {:?}", ans); // 答えを出力する
}

/**
 * 標準入力を受け取り，それをfloat32bit型のVectorとして返す
 */
fn read_vec_float32() -> Vec<f32> {
    let mut line = String::new(); // 入力を受け取るための変数を作成
    stdin().read_line(&mut line).unwrap(); // 標準入力を受け取り， line に束縛する

    return line
        .split_whitespace() // lineを空白で分割する
        // mapでイテレータを作成し，要素と順番に取り出す．parseで &str を f32 に変換し，Resutl型で返す
        // okはResult<T>，今回であればf32の値を取りだし，unwrapでResult<T>からTだけを取り出す
        .map(|e| e.parse::<f32>().ok().unwrap())
        .collect(); // collectで結果の値をまとめ，新規にVector型を作成する
}
