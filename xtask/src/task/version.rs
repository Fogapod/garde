use argh::FromArgs;

use crate::Result;

#[derive(FromArgs)]
#[argh(subcommand, name = "version", description = "Bump crate versions")]
pub struct Version {}

impl Version {
    pub fn run(self) -> Result {
        Ok(())
    }
}