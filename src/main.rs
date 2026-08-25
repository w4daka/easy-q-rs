use std::io::stdin;
fn main() {
    let input = input();

    let n: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    match first_even(&n) {
        None => println!("偶数なし"),
        Some(value) => println!("{}", value),
    }
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn first_even(numbers: &[i32]) -> Option<i32> {
    let mut ans = None;
    for element in numbers {
        if element % 2 == 0 {
            // 「参照をたどって、その i32 の値を取得して Option<i32> に入れている」
            ans = Some(*element);
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_even() {
        assert_eq!(first_even(&[1, 3, 5, 8, 10]), Some(8));
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_even(&[2, 4, 6]), Some(2));
        assert_eq!(first_even(&[]), None);
    }
}
