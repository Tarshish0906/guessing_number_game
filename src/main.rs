use std::io;

fn main() {
    println!("数を当ててごらん");

    println!("予想 =");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("読み込みに失敗しました");

    println!("入力値: {}", guess);
}
