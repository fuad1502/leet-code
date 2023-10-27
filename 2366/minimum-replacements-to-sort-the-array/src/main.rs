struct Solution {}

/*
We could solve this like an insertion sort problem, we work backwards from the end.
We partition the array into two parts, the sorted part and the unsorted part.
The sorted part starts from the end.
Each iteration, we evaluate the rightmost element from the unsorted part,
whether it's <= than the leftmost element from the sorted part.
If it is, we simply put it in the sorted part.
If it is not, we break the number down into two. We either put:
    > the leftmost element from the sorted part to the sorted part,
    and the remainding value to the unsorted part.
    > the rightmost element from the unsorted part divided by two
    to the unsorted part, and the remaining half to the sorted part.
Which one is chosen depends on whether the rightmost element from the unsorted part
divided by two is smaller than or equal to the leftmost element from the sorted part.
Actually, instead of doing the breaks multiple times, we can just "see" from the
unsorted number how many breaks we need to perform, and the last remainding number.
The difficulty lies in having the two condition for breaking the number.
Actually, there is an easy way: (s is the leftmost sorted number, u is the right most unsorted number)
> breaks = u / s - 1
> if u % s != 0 -> breaks += 1
> new leftmost sorted number:
    >> u % s or (u % s + u / s) / 2, whichever is greater
*/

impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().rev();
        let mut ordered_leftmost = match nums.next() {
            Some(v) => i64::from(v),
            None => return 0,
        };
        let mut num_breaks = 0;
        for num in nums {
            let num = i64::from(num);
            if num <= ordered_leftmost {
                ordered_leftmost = num;
                continue;
            }
            let remainder = num % ordered_leftmost;
            let dividend = num / ordered_leftmost;
            num_breaks += dividend - 1;
            let mut new_ordered_leftmost = ordered_leftmost;
            if remainder != 0 {
                num_breaks += 1;
                new_ordered_leftmost = remainder;
                while new_ordered_leftmost < ordered_leftmost {
                    let maximum_request = ordered_leftmost - new_ordered_leftmost - 1;
                    if maximum_request <= dividend {
                        new_ordered_leftmost += maximum_request;
                        ordered_leftmost -= 1;
                    } else {
                        new_ordered_leftmost += dividend;
                        ordered_leftmost -= 1;
                    };
                }
            }
            ordered_leftmost = new_ordered_leftmost;
        }
        num_breaks
    }
}

fn main() {
    // let mut nums = vec![3, 9, 3];
    // assert_eq!(Solution::minimum_replacement(nums), 2);
    // nums = vec![1, 2, 3, 4, 5];
    // assert_eq!(Solution::minimum_replacement(nums), 0);
    // nums = vec![2, 10, 20, 19, 1];
    // assert_eq!(Solution::minimum_replacement(nums), 47);
    let nums = vec![
        368, 112, 2, 282, 349, 127, 36, 98, 371, 79, 309, 221, 175, 262, 224, 215, 230, 250, 84,
        269, 384, 328, 118, 97, 17, 105, 342, 344, 242, 160, 394, 17, 120, 335, 76, 101, 260, 244,
        378, 375, 164, 190, 320, 376, 197, 398, 353, 138, 362, 38, 54, 172, 3, 300, 264, 165, 251,
        24, 312, 355, 237, 314, 397, 101, 117, 268, 36, 165, 373, 269, 351, 67, 263, 332, 296, 13,
        118, 294, 159, 137, 82, 288, 250, 131, 354, 261, 192, 111, 16, 139, 261, 295, 112, 121,
        234, 335, 256, 303, 328, 242, 260, 346, 22, 277, 179, 223,
    ];
    assert_eq!(Solution::minimum_replacement(nums), 17748);
}
