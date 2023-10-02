#![forbid(unsafe_code)]
#![warn(clippy::all)]
// #![warn(clippy::pedantic)]

use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;
use log::warn;

pub fn frobnicate(widgets: u32) -> Result<bool> {
    if widgets > 100 {
        Ok(true)
    } else if widgets > 1 {
        Ok(false)
    } else {
        Err(anyhow!("Ouch"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Ok;
    use pretty_assertions::assert_eq;

    #[test]
    fn cant_frobnicate_small_numbers() -> Result<()> {
        assert_eq!(frobnicate(7)?, false);
        Ok(())
    }

    #[test]
    fn handles_frobnication_of_big_numbers() -> Result<()> {
        assert_eq!(frobnicate(700)?, true);
        Ok(())
    }
}
