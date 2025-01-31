#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod command;
mod flow;
mod io;
#[cfg(feature = "serde")]
mod serde;
mod state;

#[doc(inline)]
pub use self::{command::Command, flow::*, io::Io, state::State};
