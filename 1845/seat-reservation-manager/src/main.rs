struct SeatManager {
    heap: Heap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager {
            heap: Heap::new((1..=n).collect()),
        }
    }

    fn reserve(&mut self) -> i32 {
        self.heap.pop()
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(seat_number);
    }
}

struct Heap<T: Ord> {
    elements: Vec<T>,
    n: usize,
}

impl<T: Ord + Copy> Heap<T> {
    fn new(elements: Vec<T>) -> Self {
        let n = elements.len();
        let mut heap = Heap { elements, n };
        for i in n / 2..n {
            heap.bubble_down(i);
        }
        heap
    }

    fn pop(self: &mut Self) -> T {
        let result = self.elements[0];
        self.elements[0] = self.elements[self.n - 1];
        self.n -= 1;
        self.bubble_down(0);
        result
    }

    fn push(self: &mut Self, item: T) {
        if self.elements.len() == self.n {
            return;
        }
        self.elements[self.n] = item;
        self.n += 1;
        self.bubble_up(self.n - 1);
    }

    fn bubble_down(self: &mut Self, idx: usize) {
        let child_idx = (idx + 1) * 2 - 1;
        let mut min_idx = idx;
        for i in child_idx..=child_idx + 1 {
            if i >= self.n {
                break;
            }
            if self.elements[i] < self.elements[min_idx] {
                min_idx = i;
            }
        }
        if min_idx != idx {
            self.elements.swap(idx, min_idx);
            self.bubble_down(min_idx);
        }
    }

    fn bubble_up(self: &mut Self, idx: usize) {
        if idx == 0 {
            return;
        }
        let parent_idx = (idx + 1) / 2 - 1;
        if self.elements[idx] < self.elements[parent_idx] {
            self.elements.swap(idx, parent_idx);
            self.bubble_up(parent_idx);
        }
    }
}

fn main() {
    let n = 5;
    let actions = [None, None, Some(2), None, None, None, None, Some(5)];
    let expected_seats = [
        Some(1),
        Some(2),
        None,
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
    ];
    let mut obj = SeatManager::new(n);
    for (action, expected_seat) in actions.iter().zip(expected_seats.iter()) {
        match action {
            None => assert_eq!(obj.reserve(), expected_seat.unwrap()),
            Some(a) => obj.unreserve(*a),
        }
    }
}
