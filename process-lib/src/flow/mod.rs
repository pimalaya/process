//! # Flows
//!
//! Module dedicated to I/O-free, iterable state machine flows.

#[path = "spawn-then-wait.rs"]
mod spawn_then_wait;
#[path = "spawn-then-wait-with-output.rs"]
mod spawn_then_wait_with_output;

#[doc(inline)]
pub use self::{
    spawn_then_wait::SpawnCommandThenWait,
    spawn_then_wait_with_output::SpawnCommandThenWaitWithOutput,
};
