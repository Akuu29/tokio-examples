use tokio::time::{interval, sleep, Duration};

async fn elapsed_counter(count: i32) {
    let time: u64 = 3;
    let time_ = &time;
    sleep(Duration::from_secs(time)).await;
    let counter = count as u64 * (time_ + 1);
    println!("{}s elapsed", counter);
}
#[tokio::main]
async fn main() {
    let mut interval = interval(Duration::from_secs(1));
    for i in 1..6 {
        // インターバルは最後のtickからの時間を測定する。
        // tick().awaitを呼び出す間に時間が経過した場合、インターバルに指定した時間より短い時間になることがある。
        interval.tick().await;
        // elapsed_counterの中でtickをsleepに置き換えている。
        // elapsed_counterは4秒に1回実行される。
        elapsed_counter(i).await;
    }
}