use tokio::time::{sleep, Duration};

async fn my_async_function() {
    for i in 0..5 {
        println!("Async function loop {}", i);
        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("Main function start");

    let my_async_thread = tokio::spawn(my_async_function());

    for i in 0..5 {
        println!("Main function loop {}", i);
        sleep(Duration::from_secs(1)).await;
    }

    my_async_thread.await.unwrap();

    println!("Main function end");
}