use std::io::stdin;
fn main() {
    let input = input();

    let n: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    match search_max(n) {
        None => println!("最大値なし"),
        Some(value) => println!("{}", value),
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn search_max(numbers: Vec<i32>) -> Option<i32> {
    let mut max = None;
    for element in numbers {
        match max {
            None => max = Some(element),
            Some(value) => {
                if value < element {
                    max = Some(element)
                }
            }
        }
    }
    max
}
