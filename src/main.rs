extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("数を当ててごらん");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("秘密の数字は... {}", secret_number);

    println!("予想 =");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("読み込みに失敗しました");

    println!("入力値: {}", guess);
}
