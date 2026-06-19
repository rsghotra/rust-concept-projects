/*
Theory: The implementation is single threaded and focused on correctness. In a production, low-latency system, I would use atomics for 
head/tail, cache-line padding to avoid false sharing, and memory ordering carefully for SPSC/MPSC

I used a fixed size vector and two indexes: head for reading and tail for writing. After each push or pop, the index advances using modulo
so it wraps around to the beginning. The len field helps distinguish full vs empty when head == tail

self.tail = (self.tail + 1) % self.capacity

Atomics to use it thread safe
Write goes at tail
read goes from head
when index reaches end, it wraps to 0

buffer = actual storage
head   = next read position
tail   = next write position
size   = current number of elements

*/


struct RingBuffer {
    buffer: Vec<Option<i32>>,
    capacity: usize,
    head: usize,
    tail: usize,
    size: usize,
}

impl RingBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            capacity,
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    fn push(&mut self, value: i32) -> bool {
        if self.size == self.capacity {
            return false;
        }

        self.buffer[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;

        true
    }

    fn pop(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None
        }

        let value = self.buffer[self.head].take();

        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;

        value
    }
}



fn main() {
    let mut rb = RingBuffer::new(3);

    rb.push(10);
    rb.push(20);
    rb.push(30);

    println!("{:?}", rb.pop()); //Some(10)

    rb.push(40);

    println!("{:?}", rb.pop());
    println!("{:?}", rb.pop());
    println!("{:?}", rb.pop());

}



