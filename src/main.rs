use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TokenBucket {
    capacity: u32,                  // 桶的总容量
    tokens: f64,                    // 桶内现在的令牌数
    refill_rate: f64,               // 添加令牌速率，例如每秒往里面添加N个令牌
    last_refill_timestamp: Instant, // 使用单调时间，防止时间调整带来的影响
}

#[allow(dead_code)]
impl TokenBucket {
    pub fn new(capacity: u32, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_rate,
            last_refill_timestamp: Instant::now(),
        }
    }

    fn refill(&mut self) {
        // 获取当前时间间隔，用于计算时间间隔
        let now = Instant::now();
        // 计算从上次补充令牌依赖，已经过去多少秒
        let elapsed = now.duration_since(self.last_refill_timestamp).as_secs_f64();

        // 计算在这段时间内应该产生多少个新令牌
        let new_tokens = elapsed * self.refill_rate;
        // 当桶内已满就丢弃后面的令牌
        self.tokens = (self.tokens + new_tokens).min(self.capacity as f64);
        self.last_refill_timestamp = now;
    }

    pub fn try_consume(&mut self, amount: u32) -> bool {
        self.refill(); // 没有使用外部计时器补充令牌，所以这里在尝试消费令牌前先主动补充令牌

        if self.tokens >= amount as f64 {
            self.tokens -= amount as f64;
            true
        } else {
            false
        }
    }
}

fn main() {
    let bucket = Arc::new(Mutex::new(TokenBucket::new(10, 5.0))); // 容量为10， 每秒5个令牌

    for i in 0..50 {
        let bucket = Arc::clone(&bucket);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 100)); // 模拟不同时间的请求
            let mut b = bucket.lock().unwrap();

            if b.try_consume(1) {
                println!("[{}] 请求成功", i);
            } else {
                println!("[{}] 请求被限流", i);
            }
        });
    }

    thread::sleep(Duration::from_secs(5));
}
