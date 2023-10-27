struct Solution {}

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut requests = 0;
        let mut ages = ages.clone();
        ages.sort();
        println!("{:?}", ages);
        for (i, age) in ages.iter().enumerate() {
            let max_age_idx = ages.partition_point(|&x| x <= *age);
            let min_age_idx =
                ages.partition_point(|&x| f64::from(x) <= (f64::from(*age) * 0.5 + 7.0));
            let mut new_request = i32::try_from(max_age_idx).expect("Partition point does not fit in i32")
                - i32::try_from(min_age_idx).expect("Partition point does not fit in i32");
            if i >= min_age_idx && i <= max_age_idx - 1{
                new_request -= 1;
            }
            if new_request > 0 {
                requests += new_request;
            }
            println!("{min_age_idx}, {max_age_idx} -> {requests}");
        }
        requests
    }
}

fn main() {
    // let ages = vec![108, 115, 5, 24, 82];
    let ages = vec![101,98,80,20,1,97,3,77,114,109];
    assert_eq!(Solution::num_friend_requests(ages), 21);
}
