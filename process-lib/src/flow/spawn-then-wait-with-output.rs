//! # Spawn then wait with output flow
//!
//! Module dedicated to the I/O-free
//! [`SpawnCommandThenWaitWithOutput`] flow.

use std::process::{Output, Stdio};

use tracing::instrument;

use crate::{Command, Io, State};

/// The I/O-free flow for spawning a process then waiting for its
/// child's output.
///
/// This flow should be used when you need to collect the child
/// process' output, from stdout and stderr.
///
/// If you do not need to collect the output, or if you need to pipe
/// the output to another process, see
/// [`super::SpawnCommandThenWait`].
#[derive(Debug)]
pub struct SpawnCommandThenWaitWithOutput {
    /// The inner I/O state used to communicate with I/O connectors.
    state: State,
}

impl SpawnCommandThenWaitWithOutput {
    /// Creates a new flow from the given command builder.
    #[instrument]
    pub fn new(command: Command) -> SpawnCommandThenWaitWithOutput {
        Self {
            state: State::new(command),
        }
    }

    /// Takes the stdin away from the flow's inner I/O state.
    #[instrument(skip_all)]
    pub fn take_stdin(&mut self) -> Option<Stdio> {
        self.state.stdin.take()
    }

    /// Takes the output away from the flow's inner I/O state.
    #[instrument(skip_all)]
    pub fn take_output(&mut self) -> Option<Output> {
        self.state.output.take()
    }
}

impl AsMut<State> for SpawnCommandThenWaitWithOutput {
    fn as_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

impl Iterator for SpawnCommandThenWaitWithOutput {
    type Item = Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.state.output.is_none() {
            Some(Io::SpawnThenWaitWithOutput)
        } else {
            None
        }
    }
}
