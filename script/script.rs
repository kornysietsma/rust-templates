#!/usr/bin/env cargo +nightly -Zscript
---cargo
[package]
edition = "2024"
[dependencies]
anyhow = "1.0.100"
itertools = "0.14.0"
derive_builder = "0.20.2"
lazy_static = "1.5.0"
clap = { version = "4.5.54", features = ["derive"] }
log = "0.4.29"
env_logger = "0.11.8"
[dev-dependencies]
pretty_assertions = "1.4.1"
---

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(rust_2024_compatibility)]
#![warn(deprecated_safe)]
use anyhow::anyhow;
use anyhow::Result;
use env_logger::Env;

use clap::Parser;
use log::info;
use log::warn;
use log::debug;

pub fn frobnicate(widgets: u32) -> Result<bool> {
    debug!("frobnicate: widgets={}", widgets);
    if widgets > 100 {
        Ok(true)
    } else if widgets > 1 {
        Ok(false)
    } else {
        Err(anyhow!("Ouch"))
    }
}

fn run(widgets: u32) -> Result<()> {
    info!("frobnicating!");
    if frobnicate(widgets)? {
        info!("success!");
    } else {
        warn!("not enough widgets to frobnicate");
    }
    Ok(())
}

#[derive(Debug, Parser)]
#[clap()]
struct Opts {
    #[clap(value_parser, short, long)]
    widgets: u32,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opts = Opts::parse();

    run(opts.widgets)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Ok;
    use pretty_assertions::assert_eq;

    #[test]
    fn handles_frobnication() -> Result<()> {
        assert_eq!(frobnicate(7)?, true);
        Ok(())
    }
}