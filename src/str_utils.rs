// calc edit distance (Levenshtein distance)
pub fn edit_distance(str1: &str, str2: &str) -> usize {
    use std::cmp::min;
    let n = str2.chars().count();
    let mut edit = (0..=n).collect::<Vec<_>>();
    let mut old_diag = 0;
    for (i, c1) in str1.chars().enumerate() {
        let mut left = i + 1;
        edit[0] = left;
        for (j, c2) in str2.chars().enumerate() {
            let new_diag = edit[j + 1];
            let rep = if c1 == c2 { old_diag } else { old_diag + 1 };
            let center = min(min(left, new_diag) + 1, rep);
            old_diag = new_diag;
            left = center;
            edit[j + 1] = center;
        }
    }
    edit[n]
}

pub fn is_digits(a: &str) -> bool {
    a.chars().all(|c| c.is_digit(10) || c == '.')
}

pub fn split_command(line: &str) -> Vec<Vec<&str>> {
    line.split('|')
        .map(|x| x.trim().split_whitespace().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{edit_distance, is_digits, split_command};
    #[test]
    fn edit_distance_test_1() {
        assert_eq!(edit_distance("hoge", "piyo"), 4);
        assert_eq!(edit_distance("hoge", "huga"), 2);
        assert_eq!(edit_distance("hoge", "hage"), 1);
        assert_eq!(edit_distance("huga", "piyo"), 4);
        assert_eq!(edit_distance("foo", "bar"), 3);
        assert_eq!(edit_distance("baz", "bar"), 1);
    }
    #[test]
    fn edit_distance_test_2() {
        assert_eq!(edit_distance("こんにちは", "こんばんは"), 2);
        assert_eq!(edit_distance("おはようございます", "こんばんは"), 9);
        assert_eq!(edit_distance("コンピューター", "コンデンサー"), 4);
    }
    #[test]
    fn edit_distance_test_3() {
        assert_eq!(
            edit_distance(
                "The quick brown fox jumps over the lazy dog.",
                "The quick onyx goblin jumps over the lazy dwarf."
            ),
            14
        );
    }
    #[test]
    fn edit_distance_test_4() {
        let a = "The quick brown fox jumps over the lazy dog.";
        let mut s = String::new();
        for _ in 0..10 {
            s += a;
        }
        assert_eq!(edit_distance(&s, &s), 0);
    }
    #[test]
    fn is_digits_test1() {
        assert!(is_digits("12345567890"));
        assert!(is_digits("00000000000"));
        assert!(is_digits("3.1415926535"));
        assert!(is_digits("0777"));
        assert!(!is_digits("6.02e23"));
        assert!(!is_digits("89abcdef"));
        assert!(!is_digits("0x1234"));
        assert!(!is_digits("DEADBEAF"));
        assert!(!is_digits("億千万"));
    }
    #[test]
    fn is_digits_test2() {
        assert!(!is_digits("bc"));
        assert!(!is_digits("cc"));
        assert!(!is_digits("cd"));
        assert!(!is_digits("dc"));
        assert!(!is_digits("dd"));
        assert!(!is_digits("df"));
        assert!(!is_digits("fc"));
    }
    #[test]
    fn split_command_test() {
        let s =
            "cat file | grep -E hoge | cut -f1 | sed -e 's/old/new/g' | sort | uniq -c | sort -n";
        let v = vec![
            vec!["cat", "file"],
            vec!["grep", "-E", "hoge"],
            vec!["cut", "-f1"],
            vec!["sed", "-e", "'s/old/new/g'"],
            vec!["sort"],
            vec!["uniq", "-c"],
            vec!["sort", "-n"],
        ];
        assert_eq!(split_command(s), v);
    }
}
