use std::time::{Duration, Instant};


struct TokenBucket {
    capacity: u32,
    tokens: u32,
    refill_amount: u32,
    refill_interval: Duration,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: u32, refill_amount: u32, refill_interval: Duration) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_amount,
            refill_interval,
            last_refill: Instant::now(),
        }
    }

    fn try_consume(&mut self) -> bool {
        let now = Instant::now();

        if now.duration_since(self.last_refill) >= self.refill_interval {
            self.tokens = std::cmp::min(self.capacity, self.tokens + self.refill_amount);

            self.last_refill = now;

        };

        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}

fn main() {
    println!("Hello, world!");
}
