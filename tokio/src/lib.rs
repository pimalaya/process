#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use std::io::{Error, ErrorKind, Result};

use process_lib::{Command, Io, State};
use tokio::process::Command as TokioCommand;
use tracing::instrument;

/// The Tokio-based, async I/O connector.
///
/// This connector makes use of the Tokio module [`tokio::process`] to
/// spawn processes and wait for exit status or output.
#[derive(Debug, Default)]
pub struct Connector;

impl Connector {
    /// Creates a new connector.
    #[instrument(skip_all)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Executes the given `io` for the given `flow`.
    #[instrument(skip_all)]
    pub async fn execute<F: AsMut<State>>(&self, flow: &mut F, io: Io) -> Result<()> {
        let state = flow.as_mut();

        match io {
            Io::SpawnThenWait => self.spawn_then_wait(state).await,
            Io::SpawnThenWaitWithOutput => self.spawn_then_wait_with_output(state).await,
        }
    }

    /// Spawns a process then wait for its child's exit status.
    ///
    /// This function builds a [`tokio::process::Command`] from the
    /// flow's command builder, spawns a process, collects
    /// std{in,out,err} then wait for the exit status.
    #[instrument(skip_all)]
    async fn spawn_then_wait(&self, state: &mut State) -> Result<()> {
        let Some(builder) = state.take_command_builder() else {
            return Err(Error::new(ErrorKind::NotFound, "missing command builder"));
        };

        let mut child = build_command(builder).spawn()?;
        state.set_some_stdin(child.stdin.take().and_then(|io| io.into_owned_fd().ok()));
        state.set_some_stdout(child.stdout.take().and_then(|io| io.into_owned_fd().ok()));
        state.set_some_stderr(child.stderr.take().and_then(|io| io.into_owned_fd().ok()));

        let status = child.wait().await?;
        state.set_status(status);

        Ok(())
    }

    /// Spawns a process then wait for its child's output.
    ///
    /// This function builds a [`tokio::process::Command`] from the
    /// flow's command builder, spawns a process, then wait for the
    /// output.
    #[instrument(skip_all)]
    async fn spawn_then_wait_with_output(&self, state: &mut State) -> Result<()> {
        let Some(builder) = state.take_command_builder() else {
            return Err(Error::new(ErrorKind::NotFound, "missing command builder"));
        };

        let output = build_command(builder).output().await?;
        state.set_output(output);

        Ok(())
    }
}

/// Maps a [`process_lib::Command`] to a [`tokio::process::Command`].
#[instrument(skip_all)]
fn build_command(builder: Command) -> TokioCommand {
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
