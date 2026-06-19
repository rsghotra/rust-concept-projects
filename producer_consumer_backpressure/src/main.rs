use std::{collections::VecDeque, ops::Bound};

struct BoundedQueue {
    queue: VecDeque<i32>,
    capacity: usize,
}

impl BoundedQueue {
    fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            capacity,
        }
    }

    fn produce(&mut self, value: i32) -> bool {
        if self.queue.len() == self.capacity {
            return false; //backpressured
        }

        self.queue.push_back(value);
        true
    }

    fn consume(&mut self) -> Option<i32> {
        self.queue.pop_front()
    }
}

fn main() {
    let mut queue = BoundedQueue::new(3);

    println!("{}", queue.produce(10)); // true
    println!("{}", queue.produce(20)); // true
    println!("{}", queue.produce(30)); // true

    // Queue full
    println!("{}", queue.produce(40)); // false

    println!("{:?}", queue.consume()); // Some(10)
    println!("{:?}", queue.consume()); // Some(20)

    // Space available again
    println!("{}", queue.produce(40)); // true

    println!("{:?}", queue.consume()); // Some(30)
    println!("{:?}", queue.consume()); // Some(40)
    println!("{:?}", queue.consume()); // None
}
