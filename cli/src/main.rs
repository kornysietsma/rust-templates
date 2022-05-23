#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
pub mod mymodule;

use anyhow::Result;
use env_logger::Env;

use log::info;
use log::warn;
use structopt::StructOpt;

use crate::mymodule::frobnicate;

fn run(widgets: u32) -> Result<()> {
    info!("frobnicating!");
    if frobnicate(widgets)? {
        info!("success!");
    }
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "{{project-name}}", about = "TODO: better description")]
struct Opts {
    #[structopt(short, long)]
    widgets: u32,
}

fn main() -> Result<()> {
    // default log level to info
    // env_logger::init();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opts = Opts::from_args_safe()?;

    run(opts.widgets)
}
