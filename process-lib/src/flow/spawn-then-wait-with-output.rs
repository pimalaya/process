use std::process::{Output, Stdio};

use tracing::instrument;

use crate::{Command, Io, State};

#[derive(Debug)]
pub struct SpawnCommandThenWaitWithOutput {
    state: State,
}

impl SpawnCommandThenWaitWithOutput {
    #[instrument]
    pub fn new(command: Command) -> SpawnCommandThenWaitWithOutput {
        Self {
            state: State::new(command),
        }
    }

    #[instrument(skip_all)]
    pub fn take_stdin(&mut self) -> Option<Stdio> {
        self.state.stdin.take()
    }

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
