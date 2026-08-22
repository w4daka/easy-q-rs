use std::io::stdin;
use std::str;
fn main() {
    let input = input();
    let split: Vec<&str> = input.split_whitespace().collect();
    let mut ans: &str = "a";
    let mut max = 0;
    for element in split {
        if element.len() > max {
            max = element.len();
            ans = element;
        }
    }
    println!("{}", ans)
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
