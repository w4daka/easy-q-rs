use std::collections::HashMap;

pub fn count_words(input: &str) -> HashMap<&str, usize> {
    let mut ans = HashMap::new();

    // elementはループのたびに1単語になる
    for element in input.split_whitespace() {
        // 単語を1つ取得
        //     ↓
        // HashMapから検索
        //     ↓
        // ┌───────────────┐
        // │ キーが存在する │
        // └───────┬───────┘
        //         ↓
        //     Some(value)
        //         ↓
        //     *value += 1
        //
        // ┌───────────────┐
        // │ キーがない     │
        // └───────┬───────┘
        //         ↓
        //       None
        //         ↓
        // insert(element)
        // get_mutはそのキーに対応する値への可変参照を取得する
        match ans.get_mut(element) {
            // Optionでキーが存在した/しなかったを表現
            Some(value) => *value += 1,
            None => {
                ans.insert(element, 1);
            }
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
