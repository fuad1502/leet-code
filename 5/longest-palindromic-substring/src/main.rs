struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max_count = 1;
        let mut longest_palindrome = &s[0..1];
        for i in 0..s.len() * 2 {
            let idx = i / 2;
            let left_chars = s[0..idx].chars().rev();
            let right_chars = match i % 2 {
                0 => s[idx..].chars(),
                _ => s[idx + 1..].chars(),
            };
            let mut count = if i % 2 == 0 { 0 } else { 1 };
            for (l, r) in left_chars.zip(right_chars) {
                if l == r {
                    count += 2;
                } else {
                    break;
                }
            }
            if count > max_count {
                max_count = count;
                longest_palindrome = match i % 2 {
                    0 => &s[idx - count / 2..idx + count / 2],
                    _ => &s[idx - count / 2..=idx + count / 2],
                }
            }
        }
        longest_palindrome.to_string()
    }
}

fn main() {
    let s = String::from("babad");
    // let s = String::from("cbbd");
    let longest_palindrome = Solution::longest_palindrome(s);
    assert_eq!(longest_palindrome, "bab");
    // assert_eq!(longest_palindrome, "bb");
}
