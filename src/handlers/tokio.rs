//! Module dedicated to the tokio-based, async I/O handler.

use std::io::Result;

use tokio::process::Command as TokioCommand;

use crate::{Command, Io, State};

/// The tokio-based, async I/O handler.
///
/// This handler makes use of the tokio module [`tokio::process`] to
/// spawn processes and wait for exit status or output.
pub async fn handle(mut flow: impl AsMut<State>, io: Io) -> Result<()> {
    match io {
        Io::SpawnThenWait => spawn_then_wait(flow.as_mut()).await,
        Io::SpawnThenWaitWithOutput => spawn_then_wait_with_output(flow.as_mut()).await,
    }
}

/// Spawns a process then wait for its child's exit status.
///
/// This function builds a [`tokio::process::Command`] from the flow's
/// command builder, spawns a process, collects std{in,out,err} then
/// waits for the exit status.
pub async fn spawn_then_wait(state: &mut State) -> Result<()> {
    let mut command = TokioCommand::from(state.take_command());
    let mut child = command.spawn()?;

    state.set_some_stdin(child.stdin.take().and_then(|io| io.into_owned_fd().ok()));
    state.set_some_stdout(child.stdout.take().and_then(|io| io.into_owned_fd().ok()));
    state.set_some_stderr(child.stderr.take().and_then(|io| io.into_owned_fd().ok()));

    let status = child.wait().await?;
    state.set_status(status);

    Ok(())
}

/// Spawns a process then wait for its child's output.
///
/// This function builds a [`tokio::process::Command`] from the flow's
/// command builder, spawns a process, then waits for the output.
pub async fn spawn_then_wait_with_output(state: &mut State) -> Result<()> {
    let mut command = TokioCommand::from(state.take_command());
    let output = command.output().await?;

    state.set_output(output);
    Ok(())
}

/// Converts a [`Command`] builder into a [`tokio::process::Command`].
impl From<Command> for TokioCommand {
    fn from(builder: Command) -> Self {
        let mut command = TokioCommand::new(builder.program);

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
