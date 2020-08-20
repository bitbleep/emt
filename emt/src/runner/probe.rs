use common::Meta;

use crate::runner::Error;

pub struct Runner {}

impl Runner {
    pub fn attach() -> Result<Self, Error> {
        Ok(Self {})
    }
}

impl crate::runner::Runner for Runner {
    fn meta(&mut self) -> Meta {
        panic!("");
    }
}
