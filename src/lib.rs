use anyhow::Result;
use clap::Parser;
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

    Ok(())
}