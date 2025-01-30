use std::process::{ExitStatus, Stdio};

use tracing::instrument;

use crate::{Command, Io, State};

#[derive(Debug)]
pub struct SpawnCommandThenWait {
    state: State,
}

impl SpawnCommandThenWait {
    #[instrument]
    pub fn new(command: Command) -> SpawnCommandThenWait {
        Self {
            state: State::new(command),
        }
    }

    #[instrument(skip_all)]
    pub fn take_stdin(&mut self) -> Option<Stdio> {
        self.state.stdin.take()
    }

    #[instrument(skip_all)]
    pub fn take_stdout(&mut self) -> Option<Stdio> {
        self.state.stdout.take()
    }

    #[instrument(skip_all)]
    pub fn take_stderr(&mut self) -> Option<Stdio> {
        self.state.stderr.take()
    }

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
