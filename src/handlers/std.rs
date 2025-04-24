//! Module dedicated to the standard, blocking I/O handler.

use std::{io::Result, process::Command as StdCommand};

use crate::{Command, Io, State};

/// The standard, blocking I/O handler.
///
/// This handler makes use of the standard module [`std::process`] to
/// spawn processes and wait for exit status or output.
pub fn handle(mut flow: impl AsMut<State>, io: Io) -> Result<()> {
    match io {
        Io::SpawnThenWait => spawn_then_wait(flow.as_mut()),
        Io::SpawnThenWaitWithOutput => spawn_then_wait_with_output(flow.as_mut()),
    }
}

/// Spawns a process then wait for its child's exit status.
///
/// This function builds a [`std::process::Command`] from the flow's
/// command builder, spawns a process, collects std{in,out,err} then
/// waits for the exit status.
pub fn spawn_then_wait(state: &mut State) -> Result<()> {
    let mut command = StdCommand::from(state.take_command());
    let mut child = command.spawn()?;

    state.set_some_stdin(child.stdin.take());
    state.set_some_stdout(child.stdout.take());
    state.set_some_stderr(child.stderr.take());

    let status = child.wait()?;
    state.set_status(status);
    Ok(())
}

/// Spawns a process then wait for its child's output.
///
/// This function builds a [`std::process::Command`] from the flow's
/// command builder, spawns a process, then waits for the output.
pub fn spawn_then_wait_with_output(state: &mut State) -> Result<()> {
    let mut command = StdCommand::from(state.take_command());
    let output = command.output()?;

    state.set_output(output);
    Ok(())
}

/// Converts a [`Command`] builder to a [`std::process::Command`].
impl From<Command> for StdCommand {
    fn from(builder: Command) -> Self {
        let mut command = StdCommand::new(builder.program);

        if let Some(args) = builder.args {
            for arg in args {
                command.arg(arg);
            }
        }

        if let Some(envs) = builder.envs {
            for (key, val) in envs {
                command.env(key, val);
            }
        }

        if let Some(dir) = builder.current_dir {
            command.current_dir(dir);
        }

        if let Some(cfg) = builder.stdin {
            command.stdin(cfg);
        }

        if let Some(cfg) = builder.stdout {
            command.stdout(cfg);
        }

        if let Some(cfg) = builder.stderr {
            command.stderr(cfg);
        }

        command
    }
}
