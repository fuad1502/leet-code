// The idea is first to create an array of how many minutes it will take for each monster to reach
// the city (dist / speed). Once we have that, we sort it in ascending order. Then we determine the
// index at which the time required for that monster to reach the city is greater than or equal to its index.

struct Solution {}

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time: Vec<i32> = dist
            .into_iter()
            .zip(speed.into_iter())
            .map(|(d, s)| (d + s - 1) / s) // see bottom note on converting floor divide to ceil
            // divide
            .collect();
        time.sort();
        for (i, t) in time.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let i = i as i32;
            if i >= *t {
                return i;
            }
        }
        time.len() as i32
    }
}

fn main() {
    let mut dist = vec![1, 3, 4];
    let mut speed = vec![1, 1, 1];
    assert_eq!(Solution::eliminate_maximum(dist, speed), 3);
    dist = vec![1, 1, 2, 3];
    speed = vec![1, 1, 1, 1];
    assert_eq!(Solution::eliminate_maximum(dist, speed), 1);
    dist = vec![3, 2, 4];
    speed = vec![5, 3, 2];
    assert_eq!(Solution::eliminate_maximum(dist, speed), 1);
    dist = vec![4, 3, 3, 3, 4];
    speed = vec![1, 1, 1, 1, 4];
    assert_eq!(Solution::eliminate_maximum(dist, speed), 3);
}

// ceil(a/b) = (a + c)/b
// ceil(a/b) = floor(a/b + c/b)
// ceil(a/b) = floor((k*b + a%b)/b + c/b)
// ceil(a/b) = floor(k + (a%b)/b + c/b) -> c = b - 1
