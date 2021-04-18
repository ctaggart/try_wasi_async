use async_executor::{Executor, Task};
use futures_lite::future;

// type Error = Box<dyn std::error::Error + Send>;
// type Result<T> = std::result::Result<T, Error>;
// use anyhow::Result;
use surf::Result;

fn main() -> Result<()> {
    // Create a new executor.
    let ex = Executor::new();

    // Spawn a task.
    let task: Task<Result<()>> = ex.spawn(async {
        println!("Hello Cameron");
        // let ip = reqwest::get("http://ip.jsontest.com/").await?.text().await?;
        // there is no reactor running, must be called from the context of a Tokio 1.x runtime
        let ip = surf::get("http://ip.jsontest.com/").await?.body_string().await?;
        println!("ip {}", &ip);
        Ok(())
    });

    // Run the executor until the task completes.
    future::block_on(ex.run(task))
}
