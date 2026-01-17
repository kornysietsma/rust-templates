#!/usr/bin/env cargo +nightly -Zscript
---cargo
[package]
edition = "2024"
[dependencies]
anyhow = "1.0.100"
derive_builder = "0.20.2"
argh = "0.1"
---

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(rust_2024_compatibility)]
#![warn(deprecated_safe)]
use anyhow::anyhow;
use anyhow::Result;

use argh::FromArgs;

pub fn frobnicate(widgets: u32) -> Result<bool> {
    println!("frobnicate: widgets={}", widgets);
    if widgets > 100 {
        Ok(true)
    } else if widgets > 1 {
        Ok(false)
    } else {
        Err(anyhow!("Ouch"))
    }
}

/// A minimal script example
#[derive(FromArgs)]
struct Opts {
    /// number of widgets to frobnicate
    #[argh(option, short = 'w')]
    widgets: u32,
}

fn main() -> Result<()> {
    let opts: Opts = argh::from_env();

    println!("frobnicating!");
    if frobnicate(opts.widgets)? {
        println!("success!");
    } else {
        println!("not enough widgets to frobnicate");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Ok;

    #[test]
    fn handles_frobnication() -> Result<()> {
        assert_eq!(frobnicate(7)?, true);
        Ok(())
    }
}