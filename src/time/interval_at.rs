use tokio::time::{interval_at, Duration, Instant};

#[tokio::main]
async fn main() {
    let start_secs: u64 = 3;
    let interval_secs: u64 = 1;
    let start = Instant::now() + Duration::from_secs(start_secs);
    let mut interval = interval_at(start, Duration::from_secs(interval_secs));
    
    for _i in 0..3 {
        interval.tick().await; // i=0時3秒、それ以外1秒
        println!("hello");
        interval.tick().await; // 1秒
        println!("world");
        interval.tick().await; // 1秒
    }
}