#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![warn(rust_2024_compatibility)]
#![warn(deprecated_safe)]
pub mod mymodule;

use anyhow::Result;

use log::info;

use crate::mymodule::frobnicate;

pub fn run(widgets: u32) -> Result<()> {
    info!("frobnicating!");
    if frobnicate(widgets)? {
        info!("success!");
    }
    Ok(())
}
