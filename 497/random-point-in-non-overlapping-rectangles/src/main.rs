use std::collections::HashMap;

use rand::Rng;

struct Solution {
    rects: Vec<Vec<i32>>,
    cumulative_probabilities: Vec<f64>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut sizes = Vec::new();
        let mut total_size = 0;
        for rect in rects.iter() {
            let size = (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            total_size += size;
            sizes.push(size);
        }
        let mut s = Self {
            rects,
            cumulative_probabilities: Vec::new(),
        };
        let mut previous_cp = 0.0;
        for size in sizes {
            let cp = previous_cp + f64::from(size) / f64::from(total_size);
            s.cumulative_probabilities.push(cp);
            previous_cp = cp;
        }
        s
    }

    fn pick(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let random_number: f64 = rng.gen();
        let mut rect_index = self.rects.len() - 1;
        for (i, &cp) in self.cumulative_probabilities.iter().enumerate() {
            if random_number < cp {
                rect_index = i;
                break;
            }
        }
        let rect = &self.rects[rect_index];
        let width = rect[2] - rect[0] + 1;
        let height = rect[3] - rect[1] + 1;
        let offset_x = rng.gen::<f64>() * f64::from(width);
        let offset_y = rng.gen::<f64>() * f64::from(height);
        let random_x = rect[0] + offset_x.floor() as i32;
        let random_y = rect[1] + offset_y.floor() as i32;
        vec![random_x, random_y]
    }
}

fn main() {
    let rects = vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]];
    // let rects = vec![
    //     vec![82918473, -57180867, 82918476, -57180863],
    //     vec![83793579, 18088559, 83793580, 18088560],
    //     vec![66574245, 26243152, 66574246, 26243153],
    //     vec![72983930, 11921716, 72983934, 11921720],
    // ];
    let mut picked_points = HashMap::new();
    let mut points = vec![];
    for rect in rects.iter() {
        for x in rect[0]..=rect[2] {
            for y in rect[1]..=rect[3] {
                points.push(vec![x, y]);
                picked_points.insert(vec![x, y], 0);
            }
        }
    }
    let s = Solution::new(rects);
    for _i in 0..10000 {
        let new_point = s.pick();
        let previous_count = picked_points.get(&new_point).unwrap();
        picked_points.insert(new_point, previous_count + 1);
    }
    println!("{:?}", picked_points);
}
