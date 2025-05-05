use std::thread;
use anyhow::{anyhow, Result};
use clap::Parser;
use core_affinity::{get_core_ids,set_for_current};
use log::debug;

#[derive(Parser,Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Number of tasks to run
    #[arg(short, long)]
    load: Option<u16>
}

pub fn thread_per_core() -> Result<()> {
    let args = Arguments::parse();
    debug!("args: {args:?}");

    let core_ids = get_core_ids().ok_or(anyhow!("No cores found"))?;
    debug!("core ids: {core_ids:?}");

    let handles = core_ids.into_iter().map(|id| {
        thread::spawn(move || {
            // Pin this thread to a single CPU core.
            let pinned = set_for_current(id);
            if pinned {
                debug!("Thread for core {id:?}");
            } else {
                debug!("Failed to pin thread to core {id:?}");
            }
        })
    }).collect::<Vec<_>>();

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    Ok(())
}