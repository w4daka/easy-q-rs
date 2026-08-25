// use std::io::stdin;
fn main() {
    // let input = input();
    //
    // let n: Vec<i32> = input
    //     .split_whitespace()
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .collect();

    let s = String::from("hello");

    let len = string_len(&s);

    println!("{}", len);
    println!("{}", s);
}

// fn input() -> String {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     input
// }
fn string_len(s: &str) -> usize {
    s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_len() {
        assert_eq!(string_len("hello"), 5);
        assert_eq!(string_len(""), 0);
        assert_eq!(string_len("rust"), 4);
    }
}
