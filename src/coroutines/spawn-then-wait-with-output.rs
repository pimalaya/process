//! Module dedicated to the I/O-free [`SpawnThenWaitWithOutput`]
//! coroutine.

use std::process::Output;

use log::debug;

use crate::{Command, Io};

/// The I/O-free coroutine for spawning a process then waiting for its
/// child's output.
///
/// This coroutine should be used when you need to collect the child
/// process' output, from stdout and stderr.
///
/// If you do not need to collect the output, or if you need to pipe
/// the output to another process, see [`super::SpawnThenWait`].
#[derive(Debug)]
pub struct SpawnThenWaitWithOutput {
    command: Option<Command>,
}

impl SpawnThenWaitWithOutput {
    /// Creates a new coroutine from the given command builder.
    pub fn new(command: Command) -> Self {
        debug!("prepare command to be spawned: {command:?}");
        let command = Some(command);
        Self { command }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<Output, Io> {
        let Some(input) = input else {
            return Err(match self.command.take() {
                Some(cmd) => Io::SpawnThenWaitWithOutput(Err(cmd)),
                None => Io::UnavailableInput,
            });
        };

        let Io::SpawnThenWaitWithOutput(output) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match output {
            Ok(output) => {
                debug!("successfully spawned command: {output:?}");
                Ok(output)
            }
            Err(io) => {
                debug!("need to spawn command");
                Err(Io::SpawnThenWaitWithOutput(Err(io)))
            }
        }
    }
}
