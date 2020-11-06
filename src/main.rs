use futures::future::{self, Future};
use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

fn main() {
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // let mut rt = tokio::runtime::Builder::new()
    //     .threaded_scheduler()
    //     .core_threads(4)
    //     .on_thread_start(|| debug!("on_thread_start()"))
    //     .build()
    //     .unwrap();

    rt.block_on(async_hello());

    let x = 42;
    let async_capture = async {
        debug!("in async_capture, x => {}", x);
    };
    rt.block_on(async_capture);

    rt.block_on(async_returns_i32());
    rt.block_on(returns_future_i32());
    rt.block_on(returns_async_block_i32());
}

async fn async_returns_i32() -> i32 {
    42
}
fn returns_future_i32() -> impl Future<Output = i32> {
    future::ready(42)
}
fn returns_async_block_i32() -> impl Future<Output = i32> {
    async { 42 }
}

async fn async_hello() {
    debug!("Hello, asynchronously!");
}
