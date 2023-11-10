// One method to solve this problem is using some kind of insertion sort method.
// Let's say we have built the array from several adjacent pairs, and we know that the two ends are
// a and b. Now, to add a new adjacent pair, we simply find where a or b exist in the remaining
// adjacent pairs and append accordingly.
//
// Using a linear search would make this algorithm perform in O(n^2) time.
//
// We can create a hash table with each element as the key, and the adjacent pair index as the
// value. With this, the solution will perform in linear time.
//
// However, a hash table will take O(n) space, which we could probably do better in.
//
// However I highly doubt we can be as fast as using a hash table, since the next fastest method
// we could implement would probably take O(logn) time. And as of now, I am not even sure how I
// could implement it with an O(logn) algorithm, since it would most probably depend on sorting
// the adjacent pairs, which does not seem to make in sense in this context.
//
// So I am going to stick with the Hash table solution.

use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map: HashMap<i32, HashSet<usize>> = HashMap::new();
        for (i, pair) in adjacent_pairs.iter().enumerate() {
            for element in pair {
                match map.get_mut(element) {
                    Some(s) => {
                        s.insert(i);
                    }
                    None => {
                        let mut s = HashSet::new();
                        s.insert(i);
                        map.insert(*element, s);
                    }
                };
            }
        }

        // Find start or end and initialize result with it
        let mut result = vec![];
        let mut pair_index = 0;
        for k in map.keys() {
            let pair_indexes = map.get(k).unwrap();
            if pair_indexes.len() == 1 {
                pair_index = *pair_indexes.iter().next().unwrap();
                let pair = &adjacent_pairs[pair_index];
                result.push(*k);
                result.push(if pair[0] == *k { pair[1] } else { pair[0] });
                break;
            }
        }
        // Update map
        map.remove(&result[0]);
        let pair_indexes = map.get_mut(&result[1]);
        match pair_indexes {
            Some(s) => {
                if s.len() == 1 {
                    map.remove(&result[1]);
                } else {
                    s.remove(&pair_index);
                }
            }
            None => {}
        }

        loop {
            let end = *result.last().unwrap();
            // Get pair index
            let pair_index = match map.get(&end) {
                Some(s) => *s.iter().next().unwrap(),
                None => break,
            };
            // Get new element
            let pair = &adjacent_pairs[pair_index];
            let new_element = if pair[0] == end { pair[1] } else { pair[0] };
            // Update map
            map.remove(&end);
            let pair_indexes = map.get_mut(&new_element);
            match pair_indexes {
                Some(s) => {
                    if s.len() == 1 {
                        map.remove(&new_element);
                    } else {
                        s.remove(&pair_index);
                    }
                }
                None => {}
            }
            // Insert new element
            result.push(new_element);
        }

        result
    }

    pub fn restore_array_unoptimized(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, pair) in adjacent_pairs.iter().skip(1).enumerate() {
            for element in pair {
                match map.get_mut(element) {
                    Some(v) => v.push(i + 1),
                    None => {
                        map.insert(*element, vec![i + 1]);
                    }
                };
            }
        }

        let mut result = vec![];
        result.push(adjacent_pairs[0][0]);
        result.push(adjacent_pairs[0][1]);

        let mut work_at_head = true;

        loop {
            let working_index = if work_at_head { 0 } else { result.len() - 1 };
            // Get pair index
            let pair_index = match map.get(&result[working_index]) {
                Some(pair_indexes) => pair_indexes[0],
                None => {
                    if work_at_head {
                        work_at_head = false;
                        continue;
                    } else {
                        break;
                    }
                }
            };
            // Get new element
            let pair = &adjacent_pairs[pair_index];
            let new_element = if pair[0] == result[working_index] {
                pair[1]
            } else {
                pair[0]
            };
            // Remove working element from map
            map.remove(&result[working_index]);
            // Remove pair_index from new element pair indexes
            let pair_indexes = match map.get_mut(&new_element) {
                Some(p) => p,
                None => continue,
            };
            if pair_indexes.len() == 1 {
                map.remove(&new_element);
            } else {
                let new_pair_indexes: Vec<usize> = pair_indexes
                    .iter()
                    .map(|i| *i)
                    .filter(|&i| i != pair_index)
                    .collect();
                map.insert(new_element, new_pair_indexes);
            }
            // Insert new element
            if work_at_head {
                result.insert(0, new_element);
            } else {
                result.push(new_element);
            }
        }

        result
    }
}

fn main() {
    let mut adjacent_pairs = vec![vec![2, 1], vec![3, 4], vec![3, 2]];
    let mut expected_result = vec![1, 2, 3, 4];
    let mut reversed_expected_result = expected_result.clone();
    reversed_expected_result.reverse();
    let mut result = Solution::restore_array(adjacent_pairs);
    assert!(result == expected_result || result == reversed_expected_result);

    adjacent_pairs = vec![vec![4, -2], vec![1, 4], vec![-3, 1]];
    expected_result = vec![-2, 4, 1, -3];
    reversed_expected_result = expected_result.clone();
    reversed_expected_result.reverse();
    result = Solution::restore_array(adjacent_pairs);
    assert!(result == expected_result || result == reversed_expected_result);
}
