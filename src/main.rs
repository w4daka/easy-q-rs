use std::io::stdin;
fn main() {
    let input = input();

    let n: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let a = &n[0];
    let b = &n[1];

    println!("{}", max_ref(a, b))
}

fn input() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn max_ref(a: &i32, b: &i32) -> i32 {
    let mut ans = *a;
    if a < b {
        ans = *b;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_ref() {
        assert_eq!(max_ref(&10, &20), 20);
        assert_eq!(max_ref(&30, &5), 30);
        assert_eq!(max_ref(&7, &7), 7);
        assert_eq!(max_ref(&-10, &-20), -10);
    }
}
