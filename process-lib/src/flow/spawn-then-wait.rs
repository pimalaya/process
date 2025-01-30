//! # Spawn then wait flow
//!
//! Module dedicated to the I/O-free [`SpawnCommandThenWait`] flow.

use std::process::{ExitStatus, Stdio};

use tracing::instrument;

use crate::{Command, Io, State};

/// The I/O-free flow for spawning a process then waiting for its
/// child's exit status.
///
/// This flow should be used when you do not care about the output, or
/// when you need the output to be piped into another process.
///
/// If you need to collect the output, see
/// [`super::SpawnCommandThenWaitWithOutput`].
#[derive(Debug)]
pub struct SpawnCommandThenWait {
    /// The inner I/O state used to communicate with I/O connectors.
    state: State,
}

impl SpawnCommandThenWait {
    /// Creates a new flow from the given command builder.
    #[instrument]
    pub fn new(command: Command) -> SpawnCommandThenWait {
        Self {
            state: State::new(command),
        }
    }

    /// Takes the stdin away from the flow's inner I/O state.
    #[instrument(skip_all)]
    pub fn take_stdin(&mut self) -> Option<Stdio> {
        self.state.stdin.take()
    }

    /// Takes the stdout away from the flow's inner I/O state.
    #[instrument(skip_all)]
    pub fn take_stdout(&mut self) -> Option<Stdio> {
        self.state.stdout.take()
    }

    /// Takes the stderr away from the flow's inner I/O state.
    #[instrument(skip_all)]
    pub fn take_stderr(&mut self) -> Option<Stdio> {
        self.state.stderr.take()
    }

    /// Takes the exit status away from the flow's inner I/O state.
    #[instrument(skip_all)]
    pub fn take_status(&mut self) -> Option<ExitStatus> {
        self.state.status.take()
    }
}

impl AsMut<State> for SpawnCommandThenWait {
    fn as_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

impl Iterator for SpawnCommandThenWait {
    type Item = Io;

    #[instrument(skip_all)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.state.status.is_none() {
            Some(Io::SpawnThenWait)
        } else {
            None
        }
    }
}
