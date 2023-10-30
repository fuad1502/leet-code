struct Solution {}

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr.clone();
        arr.sort();
        arr.sort_by_key(|v| Self::number_of_bits(v));
        arr
    }

    fn number_of_bits(v: &i32) -> i32 {
        let mut result = 0;
        for i in 0..31 {
            result += (v >> i) & 0x1;
        }
        result
    }
}
fn main() {
    assert_eq!(
        Solution::sort_by_bits([0, 1, 2, 3, 4, 5, 6, 7, 8].to_vec()),
        [0, 1, 2, 4, 8, 3, 5, 6, 7].to_vec()
    );
    assert_eq!(
        Solution::sort_by_bits([1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1].to_vec()),
        [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024].to_vec()
    );
}
