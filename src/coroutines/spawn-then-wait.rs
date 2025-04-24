//! Module dedicated to the I/O-free [`SpawnThenWait`] coroutine.

use log::debug;

use crate::{Command, Io, SpawnOutput};

/// The I/O-free coroutine for spawning a process then waiting for its
/// child's exit status.
///
/// This coroutine should be used when you do not care about the
/// output, or when you need the output to be piped into another
/// process.
///
/// If you need to collect the output, have a look at
/// [`super::SpawnThenWaitWithOutput`].
#[derive(Debug)]
pub struct SpawnThenWait {
    command: Option<Command>,
}

impl SpawnThenWait {
    /// Creates a new coroutine from the given command builder.
    pub fn new(command: Command) -> SpawnThenWait {
        debug!("prepare command to be spawned: {command:?}");
        let command = Some(command);
        Self { command }
    }

    /// Makes the coroutine progress.
    pub fn resume(&mut self, input: Option<Io>) -> Result<SpawnOutput, Io> {
        let Some(input) = input else {
            return Err(match self.command.take() {
                Some(cmd) => Io::SpawnThenWait(Err(cmd)),
                None => Io::UnavailableInput,
            });
        };

        let Io::SpawnThenWait(output) = input else {
            return Err(Io::UnexpectedInput(Box::new(input)));
        };

        match output {
            Ok(output) => {
                debug!("successfully spawned command: {output:?}");
                Ok(output)
            }
            Err(io) => {
                debug!("need to spawn command");
                Err(Io::SpawnThenWait(Err(io)))
            }
        }
    }
}
