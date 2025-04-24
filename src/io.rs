use std::process::Output;

use crate::{Command, SpawnOutput};

/// The process I/O request enum, emitted by flows and processed by
/// handlers.
///
/// This enum represents all the possible I/O requests that a process
/// flow can emit. I/O handlers should be able to handle all variants.
#[derive(Debug)]
pub enum Io {
    UnavailableInput,
    UnexpectedInput(Box<Io>),

    /// I/O for spawning a process and waiting for its exit status.
    ///
    /// This variant requires I/O connectors to take the command
    /// builder from the flow using [`take_command`], spawn a process,
    /// give child process' stdio to the flow using [`set_stdin`],
    /// [`set_stdout`] and [`set_stderr`], and finally give the child
    /// process' [`ExitStatus`] to the flow using [`set_status`].
    ///
    /// [`take_command`]: crate::State::take_command
    /// [`set_stdin`]: crate::State::set_stdin
    /// [`set_stdout`]: crate::State::set_stdout
    /// [`set_stderr`]: crate::State::set_stderr
    /// [`ExitStatus`]: std::process::ExitStatus
    /// [`set_status`]: crate::State::set_status
    SpawnThenWait(Result<SpawnOutput, Command>),

    /// I/O for spawning a process and waiting for its exit status and
    /// any potential output from stdout or stderr.
    ///
    /// This variant requires I/O connectors to take the command
    /// builder from the flow using [`take_command`], spawn a process
    /// then give the child process' [`Output`] to the flow using
    /// [`set_output`].
    ///
    /// [`take_command`]: crate::State::take_command
    /// [`Output`]: std::process::Output
    /// [`set_output`]: crate::State::set_output
    SpawnThenWaitWithOutput(Result<Output, Command>),
}
