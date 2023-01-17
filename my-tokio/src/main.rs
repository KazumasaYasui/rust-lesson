use std::time::Duration;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("Hello, world! got value from the server. result={:?}", result);

    woke_up().await;

    Ok(())
}

async fn woke_up() {
    tokio::task::spawn(async{
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("woke up!");
    });

    std::thread::sleep(Duration::from_secs(5));
    println!("Done");
}
