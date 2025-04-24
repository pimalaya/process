#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod command;
pub mod flows;
pub mod handlers;
mod io;
#[cfg(feature = "serde")]
mod serde;
mod state;

#[doc(inline)]
pub use self::{command::Command, io::Io, state::State};
