struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Handle zero length string and/or pattern
        if s.len() == 0 && p.len() == 0 {
            return true;
        } else if p.len() == 0 {
            return false;
        }

        // Consume non-* head and tail tokens
        let mut unconsumed_string = &s[..];
        let mut tokens = Self::pattern_to_tokens(&p);
        match Self::consume_head_and_tail(unconsumed_string, &mut tokens) {
            Ok(s) => unconsumed_string = s,
            Err(_) => return false,
        }

        // Consume the rest of non-* tokens
        for token in tokens {
            if token == "*" {
                continue;
            } else {
                match Self::consume_first_occurance(unconsumed_string, &token) {
                    Ok(s) => unconsumed_string = s,
                    Err(_) => return false,
                }
            }
        }
        true
    }

    fn consume_head_and_tail<'a>(s: &'a str, tokens: &mut Vec<String>) -> Result<&'a str, ()> {
        let mut unconsumed_string = s;
        let first_token = tokens.first().expect("Empty tokens");
        let last_token = tokens.last().expect("Empty tokens");
        if first_token != "*" {
            match Self::consume_head(unconsumed_string, first_token) {
                Ok(s) => unconsumed_string = s,
                Err(_) => return Err(()),
            }
            // Handle single non-* token
            if tokens.len() == 1 && unconsumed_string.len() != 0 {
                return Err(());
            }
        }
        // In case of multi token, check tail too
        if tokens.len() != 1 && last_token != "*" {
            match Self::consume_tail(unconsumed_string, last_token) {
                Ok(s) => unconsumed_string = s,
                Err(_) => return Err(()),
            }
        }
        tokens.remove(0);
        tokens.pop();
        Ok(unconsumed_string)
    }

    fn pattern_to_tokens(p: &str) -> Vec<String> {
        let mut result = vec![];
        let mut is_acc = false;
        let mut acc_string = String::new();
        for c in p.chars() {
            if c == '*' {
                if is_acc {
                    result.push(acc_string.clone());
                    is_acc = false;
                }
                result.push(c.to_string());
            } else if is_acc {
                acc_string += &c.to_string();
            } else {
                acc_string = c.to_string();
                is_acc = true;
            }
        }
        if is_acc {
            result.push(acc_string.clone());
        }
        result
    }

    fn consume_head<'a>(s: &'a str, token: &str) -> Result<&'a str, ()> {
        if s.len() < token.len() {
            return Err(());
        }
        for (t, c) in token.chars().zip(s.chars()) {
            if t != '?' && t != c {
                return Err(());
            }
        }
        Ok(&s[token.len()..])
    }

    fn consume_tail<'a>(s: &'a str, token: &str) -> Result<&'a str, ()> {
        if s.len() < token.len() {
            return Err(());
        }
        for (t, c) in token.chars().rev().zip(s.chars().rev()) {
            if t != '?' && t != c {
                return Err(());
            }
        }
        Ok(&s[..s.len() - token.len()])
    }

    fn consume_first_occurance<'a>(s: &'a str, token: &str) -> Result<&'a str, ()> {
        for i in 0..s.len() {
            match Self::consume_head(&s[i..], token) {
                Ok(s) => return Ok(s),
                Err(_) => continue,
            }
        }
        Err(())
    }
}

fn main() {
    let s = String::from("adceb");
    let p = String::from("*a*b");
    // let s = String::from("aa");
    // let p = String::from("aa");
    // let s = String::from("a");
    // let p = String::from("");
    assert!(Solution::is_match(s, p));
}
