use std::io::stdin;
fn main() {
    let input = input();

    let n: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut max: i32 = 0;
    for element in n {
        if element > max {
            max = element;
        }
    }
    println!("{}", max);
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
