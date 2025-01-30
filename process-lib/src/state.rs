use std::process::{ExitStatus, Output, Stdio};

use tracing::instrument;

use crate::Command;

#[derive(Debug)]
pub struct State {
    pub(crate) command: Option<Command>,
    pub(crate) status: Option<ExitStatus>,
    pub(crate) output: Option<Output>,
    pub(crate) stdin: Option<Stdio>,
    pub(crate) stdout: Option<Stdio>,
    pub(crate) stderr: Option<Stdio>,
}

impl State {
    #[instrument(skip_all)]
    pub fn new(command: Command) -> State {
        Self {
            command: Some(command),
            status: None,
            output: None,
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }

    #[instrument(skip_all)]
    pub fn take_command_builder(&mut self) -> Option<Command> {
        self.command.take()
    }

    #[instrument(skip_all)]
    pub fn set_status(&mut self, status: ExitStatus) {
        self.status = Some(status);
    }

    #[instrument(skip_all)]
    pub fn set_output(&mut self, output: Output) {
        self.output = Some(output);
    }

    #[instrument(skip_all)]
    pub fn set_stdin(&mut self, stdin: impl Into<Stdio>) {
        self.set_some_stdin(Some(stdin));
    }

    #[instrument(skip_all)]
    pub fn set_some_stdin(&mut self, stdin: Option<impl Into<Stdio>>) {
        if let Some(stdin) = stdin {
            self.stdin = Some(stdin.into());
        }
    }

    #[instrument(skip_all)]
    pub fn set_stdout(&mut self, stdout: impl Into<Stdio>) {
        self.set_some_stdout(Some(stdout));
    }

    #[instrument(skip_all)]
    pub fn set_some_stdout(&mut self, stdout: Option<impl Into<Stdio>>) {
        if let Some(stdout) = stdout {
            self.stdout = Some(stdout.into());
        }
    }

    #[instrument(skip_all)]
    pub fn set_stderr(&mut self, stderr: impl Into<Stdio>) {
        self.set_some_stderr(Some(stderr));
    }

    #[instrument(skip_all)]
    pub fn set_some_stderr(&mut self, stderr: Option<impl Into<Stdio>>) {
        if let Some(stderr) = stderr {
            self.stderr = Some(stderr.into());
        }
    }
}
