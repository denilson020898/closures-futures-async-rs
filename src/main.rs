use futures::future::{self, Future};
use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::pin::Pin;

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

    // rt.enter(|| {
    //     debug!("in rt.enter()");
    //     tokio::spawn(future::lazy(|_| debug!("in tokio::spawn()")));
    // });
    // // tokio::spawn(future::lazy(|_| println!("in tokio::spawn")));
    // rt.spawn(future::lazy(|_| debug!("in rt.spawn()")));

    // rt.block_on(future::lazy(|_| debug!("in rt.block_on()")));
    let result = rt.block_on(future::ready("hello from rt.block_on()"));
    debug!("{}", result);

    let result = rt.block_on(returns_future_i32());
    debug!("{}", result);

    let result = rt.block_on(returns_dyn_future_i32());
    debug!("{}", result);
}

fn returns_future_i32() -> impl Future<Output = i32> {
    future::ready(42)
}

// fn returns_impl_future_i32() -> impl Future<Output = i32> {
//     if rand::random() {
//         return future::ready(42);
//     }
//     future::lazy(|_| 1337)
// }

fn returns_dyn_future_i32() -> Pin<Box<dyn Future<Output = i32>>> {
    if rand::random() {
        Box::pin(future::ready(42))
    } else {
        Box::pin(future::lazy(|_| 1337))
    }
}
