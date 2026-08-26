use std::collections::HashMap;

pub fn count_words(input: &str) -> HashMap<&str, usize> {
    let mut ans = HashMap::new();
    let split_input = input.split_whitespace();

    // elementはループのたびに1単語になる
    for element in split_input {
        if let Some(one) = ans.get_mut(element) {
            *one += 1;
        } else {
            ans.insert(element, 1);
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_single_word() {
        let result = count_words("rust");

        assert_eq!(result.get("rust"), Some(&1));
    }

    #[test]
    fn counts_repeated_words() {
        let result = count_words("rust rust rust");

        assert_eq!(result.get("rust"), Some(&3));
    }

    #[test]
    fn counts_multiple_words() {
        let result = count_words("rust is fast");

        assert_eq!(result.get("rust"), Some(&1));
        assert_eq!(result.get("is"), Some(&1));
        assert_eq!(result.get("fast"), Some(&1));
    }

    #[test]
    fn counts_each_word_independently() {
        let result = count_words("rust is fast and rust is safe");

        assert_eq!(result.get("rust"), Some(&2));
        assert_eq!(result.get("is"), Some(&2));
        assert_eq!(result.get("fast"), Some(&1));
        assert_eq!(result.get("and"), Some(&1));
        assert_eq!(result.get("safe"), Some(&1));
    }

    #[test]
    fn distinguishes_uppercase_and_lowercase() {
        let result = count_words("Rust rust RUST");

        assert_eq!(result.get("Rust"), Some(&1));
        assert_eq!(result.get("rust"), Some(&1));
        assert_eq!(result.get("RUST"), Some(&1));
    }

    #[test]
    fn handles_empty_input() {
        let result = count_words("");

        assert!(result.is_empty());
    }
}
