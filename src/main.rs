use std::time::Duration;

use async_io::Timer;
use async_executor::LocalExecutor;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let executor = LocalExecutor::new();

    async_io::block_on(executor.run(async {
        loop {
            Timer::after(Duration::from_secs(5)).await;
            log::info!("Hello, loop!");
        }
    }))
}
