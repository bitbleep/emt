use common::{Meta, Test};

use crate::runner::Error;

pub struct Runner {}

impl Runner {
    pub fn attach() -> Result<Self, Error> {
        Ok(Self {})
    }
}

impl crate::runner::Runner for Runner {
    fn meta(&mut self) -> Meta {
        unimplemented!();
    }

    fn start(&mut self, _id: u32) -> Result<Test, Error> {
        unimplemented!();
    }
}
