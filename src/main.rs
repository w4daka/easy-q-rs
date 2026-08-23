use std::io::stdin;
fn main() {
    let input = input();

    let n: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut ans: Option<i32> = None;
    for element in n {
        if element % 2 == 0 {
            ans = Some(element);
            break;
        }
    }

    match ans {
        None => println!("偶数なし"),
        Some(value) => println!("{}", value),
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
