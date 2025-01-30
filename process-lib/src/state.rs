//! # I/O state
//!
//! Module dedicated to the I/O [`State`] struct.

use std::process::{ExitStatus, Output, Stdio};

use tracing::instrument;

use crate::Command;

/// The I/O state.
///
/// This struct represents the I/O state used by I/O connectors to
/// take and set data. It is usually held by flows themselves, and
/// serve as communication bridge between flows and I/O connectors.
#[derive(Debug)]
pub struct State {
    /// The [`Command`] builder input.
    pub(crate) command: Option<Command>,

    /// The child process' [`ExitStatus`] output.
    pub(crate) status: Option<ExitStatus>,

    /// The child process' [`Output`] output.
    pub(crate) output: Option<Output>,

    /// The child process' standard input [`Stdio`] output.
    pub(crate) stdin: Option<Stdio>,

    /// The child process' standard output [`Stdio`] output.
    pub(crate) stdout: Option<Stdio>,

    /// The child process' standard error [`Stdio`] output.
    pub(crate) stderr: Option<Stdio>,
}

impl State {
    /// Builds a new state from the given [`Command`].
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

    /// Takes the command builder out of the state.
    #[instrument(skip_all)]
    pub fn take_command_builder(&mut self) -> Option<Command> {
        self.command.take()
    }

    /// Sets the given child process's exit status code.
    #[instrument(skip_all)]
    pub fn set_status(&mut self, status: ExitStatus) {
        self.status = Some(status);
    }

    /// Sets the given child process's output.
    #[instrument(skip_all)]
    pub fn set_output(&mut self, output: Output) {
        self.output = Some(output);
    }

    /// Sets the given child process's standard input.
    #[instrument(skip_all)]
    pub fn set_stdin(&mut self, stdin: impl Into<Stdio>) {
        self.set_some_stdin(Some(stdin));
    }

    /// Sets the optional child process's standard input.
    #[instrument(skip_all)]
    pub fn set_some_stdin(&mut self, stdin: Option<impl Into<Stdio>>) {
        if let Some(stdin) = stdin {
            self.stdin = Some(stdin.into());
        }
    }

    /// Sets the given child process's standard output.
    #[instrument(skip_all)]
    pub fn set_stdout(&mut self, stdout: impl Into<Stdio>) {
        self.set_some_stdout(Some(stdout));
    }

    /// Sets the optional child process's standard output.
    #[instrument(skip_all)]
    pub fn set_some_stdout(&mut self, stdout: Option<impl Into<Stdio>>) {
        if let Some(stdout) = stdout {
            self.stdout = Some(stdout.into());
        }
    }

    /// Sets the given child process's standard error.
    #[instrument(skip_all)]
    pub fn set_stderr(&mut self, stderr: impl Into<Stdio>) {
        self.set_some_stderr(Some(stderr));
    }

    /// Sets the optional child process's standard error.
    #[instrument(skip_all)]
    pub fn set_some_stderr(&mut self, stderr: Option<impl Into<Stdio>>) {
        if let Some(stderr) = stderr {
            self.stderr = Some(stderr.into());
        }
    }
}
