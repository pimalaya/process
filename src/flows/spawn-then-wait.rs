//! Module dedicated to the I/O-free [`SpawnThenWait`] flow.

use std::process::{ExitStatus, Stdio};

use crate::{Command, Io, State};

/// The I/O-free flow for spawning a process then waiting for its
/// child's exit status.
///
/// This flow should be used when you do not care about the output, or
/// when you need the output to be piped into another process.
///
/// If you need to collect the output, see
/// [`super::SpawnThenWaitWithOutput`].
#[derive(Debug)]
pub struct SpawnThenWait {
    state: State,
}

impl SpawnThenWait {
    /// Creates a new flow from the given command builder.
    pub fn new(command: Command) -> SpawnThenWait {
        Self {
            state: State::new(command),
        }
    }

    /// Takes the stdin away from the flow's inner I/O state.
    pub fn take_stdin(&mut self) -> Option<Stdio> {
        self.state.stdin.take()
    }

    /// Takes the stdout away from the flow's inner I/O state.
    pub fn take_stdout(&mut self) -> Option<Stdio> {
        self.state.stdout.take()
    }

    /// Takes the stderr away from the flow's inner I/O state.
    pub fn take_stderr(&mut self) -> Option<Stdio> {
        self.state.stderr.take()
    }

    /// Makes the flow progress.
    pub fn next(&mut self) -> Result<ExitStatus, Io> {
        match self.state.status.take() {
            Some(status) => Ok(status),
            None => Err(Io::SpawnThenWait),
        }
    }
}

impl AsMut<State> for SpawnThenWait {
    fn as_mut(&mut self) -> &mut State {
        &mut self.state
    }
}
