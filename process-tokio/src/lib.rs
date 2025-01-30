#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use std::io::{Error, ErrorKind, Result};

use process_lib::{Command, Io, State};
use tokio::process::Command as TokioCommand;
use tracing::instrument;

#[derive(Debug, Default)]
pub struct Connector;

impl Connector {
    #[instrument(skip_all)]
    pub fn new() -> Self {
        Self::default()
    }

    #[instrument(skip_all)]
    pub async fn execute<F: AsMut<State>>(&self, flow: &mut F, io: Io) -> Result<()> {
        let state = flow.as_mut();

        match io {
            Io::SpawnThenWait => self.spawn_then_wait(state).await,
            Io::SpawnThenWaitWithOutput => self.spawn_then_wait_with_output(state).await,
        }
    }

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
