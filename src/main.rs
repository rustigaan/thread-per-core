use log::{error, info};
use std::error::Error;
use thread_per_core::thread_per_core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Thread per core starting");

    if let Err(error) = thread_per_core() {
        error!("Error: {error:?}");
    }

    Ok(())
}
