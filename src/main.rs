use futures::future;
use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};

fn main() {
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);

    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    let mut rt = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(4)
        .on_thread_start(|| debug!("on_thread_start()"))
        .build()
        .unwrap();

    rt.enter(|| {
        debug!("in rt.enter()");
        tokio::spawn(future::lazy(|_| debug!("in tokio::spawn()")));
    });
    // tokio::spawn(future::lazy(|_| println!("in tokio::spawn")));
    rt.spawn(future::lazy(|_| debug!("in rt.spawn()")));

    rt.block_on(future::lazy(|_| debug!("in rt.block_on()")));
}
