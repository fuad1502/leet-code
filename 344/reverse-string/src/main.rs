struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let sz = s.len();
        for i in 0..s.len() / 2 {
            s.swap(i, sz - 1 - i);
        }
    }
}

fn main() {
    let mut s = ['H', 'e', 'l', 'l', 'o'].to_vec();
    Solution::reverse_string(&mut s);
    println!("{:?}", s);
    s = ['H', 'a', 'n', 'n', 'a', 'h'].to_vec();
    Solution::reverse_string(&mut s);
    println!("{:?}", s);
}
