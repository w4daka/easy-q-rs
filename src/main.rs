use std::io::stdin;
fn main() {
    let input = input();

    let n = input.trim().parse::<i32>().unwrap();
    let mut sum: i32 = 0;
    for element in 1..=n {
        if element % 3 == 0 || element % 5 == 0 {
            sum += element;
        }
    }
    println!("{}", sum);
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
