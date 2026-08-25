// use std::io::stdin;
fn main() {
    // let input = input();
    //
    // let n: Vec<i32> = input
    //     .split_whitespace()
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect();

    let s = String::from("hello");
    take_string2(&s);
}

// fn input() -> String {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input
// }
// Stringを所有して渡す->move->元の変数は使えない
// Stringを借りて渡すー>borrowー>元の変数を使える
// 独立したStringが必要 -> clone ->別のStringを作る
fn take_string1(s: String) -> String {
    s
}

fn take_string2(s: &str) {
    println!("{}", s)
}

fn take_string3(s: &String) -> String {
    s.clone()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_max_ref() {
//         assert_eq!(max_ref(&10, &20), 20);
//         assert_eq!(max_ref(&30, &5), 30);
//         assert_eq!(max_ref(&7, &7), 7);
//         assert_eq!(max_ref(&-10, &-20), -10);
//     }
// }
