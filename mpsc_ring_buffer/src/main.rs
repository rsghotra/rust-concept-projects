
use std::{intrinsic::AtomicOrdering::Relaxed, sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering}};


struct Slot {
    //AtomicI32 is used because producer and consuer access different threads
    value: AtomicI32,

    //false = no vald value yet
    //true = producer has finished writing value
    ready: AtomicBool,
}

struct MpscRingBuffer {
    //fixed size storage. Slots are reused again and again
    buffer: Vec<Slot>,
    capacity: usize,

    //head = next position consumer will read
    //only one consumer advances thsi
    head: AtomicUsize,

    //tail = next position producer will reserve
    //multiple producers complete to advance this
    tail: AtomicUsize,
}

impl MpscRingBuffer {
    fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);

        //precreate fixed number of slots
        //this makes the queue bounded and avoids allocaion during push/pop

        for _ in 0..capacity {
            buffer.push(Slot {
                value: AtomicI32::new(0),
                ready: AtomicBool::new(false),
            });
        }

        Self {
            buffer,
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    fn push(&self, value: i32) -> Result<(), i32> {
        loop {
            let tail = self.tail.load(Ordering::Relaxed);
            let head = self.head.load(Ordering::Acquire);


            //Queue is full
            if tail - head >= self.capacity {
                return Err(value);
            }

            //try to reserve one uniwue slot
            //only one producer can successfully move tail from old value to old + 1

            let reserved = self.tail.compare_exchange(tail, tail + 1, Ordering::AcqRel, Ordering::Relaxed);
            
            if reserved.is_ok() {
                let index = tail % self.capacity;

                //write value into the reserved slot
                self.buffer[index].value.store(value, Ordering::Relaxed);

                //mark slot ready for consumer
                //Release means: value erote happend before ready = true
                self.buffer[index].ready.store(true, Ordering::Release);
                return Ok(());
            }

            //if CAS failed, another producer reserved the slot first
            //Retry and get a new tail value

        }
    }

    fn pop(&self) -> Option<i32> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);


        if head >= tail {
            return None;
        }

        let index = head % self.capacity;

        //producer may have reserved this slot but not finished writing yet

        if !self.buffer[index].ready.load(Ordering::Acquire) {
            return None;
        }

        //read value
        let value = self.buffer[index].value.load(Ordering::Relaxed);

        //mark slot empty resuable
        self.buffer[index].ready.store(false, Ordering::Release);

        //single consumer, so normal store is enough. No CAS needed

        self.head.store(head + 1, Ordering::Release);

        Some(value)

    }
}



fn main() {

    let queue = MpscRingBuffer::new(3);

    assert_eq!(queue.push(10), Ok(()));
    assert_eq!(queue.push(20), Ok(()));
    assert_eq!(queue.push(30), Ok(()));

    // Full now.
    assert_eq!(queue.push(40), Err(40));

    assert_eq!(queue.pop(), Some(10));
    assert_eq!(queue.pop(), Some(20));

    // Space available again.
    assert_eq!(queue.push(40), Ok(()));

    assert_eq!(queue.pop(), Some(30));
    assert_eq!(queue.pop(), Some(40));
    assert_eq!(queue.pop(), None);
}
