use std::io::stdin;

use easy_rs::count_words;
fn main() {
    let input = input();
    //
    // let n =
    // input.split_whitespace();
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect();
    let result = count_words(&input);
    for (key, value) in &result {
        println!("{}: {}", key, value);
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
