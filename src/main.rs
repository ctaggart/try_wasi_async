use async_executor::{Executor, Task};
use futures_lite::future;
use surf::Result;

fn main() -> Result<()> {
    let ex = Executor::new();
    let task: Task<Result<()>> = ex.spawn(async {
        let ip = surf::get("http://ip.jsontest.com/").await?.body_string().await?;
        println!("ip {}", &ip);
        Ok(())
    });
    future::block_on(ex.run(task))
}
