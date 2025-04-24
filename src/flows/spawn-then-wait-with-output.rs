//! Module dedicated to the I/O-free [`SpawnThenWaitWithOutput`] flow.

use std::process::{Output, Stdio};

use crate::{Command, Io, State};

/// The I/O-free flow for spawning a process then waiting for its
/// child's output.
///
/// This flow should be used when you need to collect the child
/// process' output, from stdout and stderr.
///
/// If you do not need to collect the output, or if you need to pipe
/// the output to another process, see [`super::SpawnThenWait`].
#[derive(Debug)]
pub struct SpawnThenWaitWithOutput {
    state: State,
}

impl SpawnThenWaitWithOutput {
    /// Creates a new flow from the given command builder.
    pub fn new(command: Command) -> SpawnThenWaitWithOutput {
        Self {
            state: State::new(command),
        }
    }

    /// Takes the stdin away from the flow's inner I/O state.
    pub fn take_stdin(&mut self) -> Option<Stdio> {
        self.state.stdin.take()
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<Output, Io> {
        match self.state.output.take() {
            Some(output) => Ok(output),
            None => Err(Io::SpawnThenWaitWithOutput),
        }
    }
}

impl AsMut<State> for SpawnThenWaitWithOutput {
    fn as_mut(&mut self) -> &mut State {
        &mut self.state
    }
}
