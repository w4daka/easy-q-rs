use std::io::stdin;
fn main() {
    let input = input();

    let n: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut max: Option<i32> = None;
    for element in n {
        match max {
            None => max = Some(element),
            Some(value) => {
                if element < value {
                    max = Some(element);
                }
            }
        }
    }

    match max {
        Some(value) => println!("{}", value),
        None => println!("最小値なし"),
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
