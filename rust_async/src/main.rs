use std::time::Duration;

use timer_future::TimerFuture;

#[path = "timer_future.rs"]
#[allow(dead_code)]
mod timer_future;

fn main() {
    TimerFuture::new(Duration::new(1, 1));
}
