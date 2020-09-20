// use common::{/*link, link::Event,*/ runtime::Meta, test::Context};

// use crate::runner::Error;

// pub struct Runner {
//     base_address: usize,
// }

// impl Runner {
//     pub fn attach(base_address: usize) -> Result<Self, Error> {
//         Ok(Self {
//             base_address,
//             // meta: Meta {},
//         })
//     }
// }

// impl crate::runner::Runner for Runner {
//     fn meta(&mut self) -> Meta {
//         unimplemented!();
//     }

//     fn start(&mut self, _id: u32) -> Result<Context, Error> {
//         // let none_event = link::Event::None;
//         // self.link.send(none_event)?;
//         unimplemented!();
//     }
// }
