struct Solution {}

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        let mut n = n;
        let mut res = String::new();
        let mut count = 0;
        while n > 0 {
            if count % 3 == 0 && count != 0 {
                res = (n % 10).to_string() + "." + &res; 
            } else {
                res = (n % 10).to_string() + &res; 
            }
            n /= 10;
            count += 1;
        }
        res
    }
}

fn main() {
    println!("{}", Solution::thousand_separator(0));
    println!("{}", Solution::thousand_separator(1234067));
}
