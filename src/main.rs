use log::info;
use std::error::Error;
use thread_per_core::thread_per_core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Thread per core starting");

    thread_per_core()?;

    Ok(())
}
