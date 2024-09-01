use std::time::Duration;

use cached::proc_macro::cached;
use tokio::time::sleep;

#[cached(time = 10)]
async fn fetch_data(id: u32) -> String {
    println!("Fetching data for ID: {}", id);
    sleep(Duration::from_secs(2)).await;
    format!("Data for ID: {}", id)
}

#[tokio::main]
async fn main() {
    let data1 = fetch_data(1).await;
    println!("Received: {}", data1);

    let data2 = fetch_data(1).await;
    println!("Received: {}", data2);

    sleep(Duration::from_secs(11)).await;

    let data3 = fetch_data(1).await;
    println!("Received: {}", data3);
}
