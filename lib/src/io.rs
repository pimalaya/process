//! # I/O
//!
//! Module dedicated to the [`Io`] enum.

/// The I/O enum.
///
/// This enum represents all the possible I/O requests that can be
/// emitted by flows [`Iterator`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Io {
    /// I/O for spawning a process and waiting for its exit status.
    ///
    /// This variant requires I/O connectors to take the command
    /// builder from the flow using [`take_command_builder`], spawn a
    /// process, give child process' stdio to the flow using
    /// [`set_stdin`], [`set_stdout`] and [`set_stderr`], and finally
    /// give the child process' [`ExitStatus`] to the flow using
    /// [`set_status`].
    ///
    /// [`take_command_builder`]: crate::State::take_command_builder
    /// [`set_stdin`]: crate::State::set_stdin
    /// [`set_stdout`]: crate::State::set_stdout
    /// [`set_stderr`]: crate::State::set_stderr
    /// [`ExitStatus`]: std::process::ExitStatus
    /// [`set_status`]: crate::State::set_status
    SpawnThenWait,

    /// I/O for spawning a process and waiting for its exit status and
    /// any potential output from stdout or stderr.
    ///
    /// This variant requires I/O connectors to take the command
    /// builder from the flow using [`take_command_builder`], spawn a
    /// process then
    /// give the child process' [`Output`] to the flow using
    /// [`set_output`].
    ///
    /// [`take_command_builder`]: crate::State::take_command_builder
    /// [`Output`]: std::process::Output
    /// [`set_output`]: crate::State::set_output
    SpawnThenWaitWithOutput,
}
