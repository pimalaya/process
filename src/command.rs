//! Module dedicated to the [`Command`] builder.

use std::{collections::HashMap, ffi::OsString, path::PathBuf, process::Stdio};

/// The command builder.
///
/// The aim of this builder is to be able to declare a command using
/// the same API from [`std::process::Command`], without any I/O
/// interaction. I/O connectors can then take data from this builder
/// to build I/O-specific commands.
///
/// Refs: [`std::process::Command`]
#[derive(Debug, Default)]
pub struct Command {
    /// Path to the program.
    ///
    /// Refs: [`std::process::Command::get_program`]
    pub program: OsString,

    /// Arguments that will be passed to the program.
    ///
    /// Refs: [`std::process::Command::get_args`]
    pub args: Option<Vec<OsString>>,

    /// Environment variables explicitly set for the child process.
    ///
    /// Refs: [`std::process::Command::get_envs`]
    pub envs: Option<HashMap<OsString, OsString>>,

    /// Working directory of the child process.
    ///
    /// Refs: [`std::process::Command::get_current_dir`]
    pub current_dir: Option<PathBuf>,

    /// Configuration for the child process's standard input (stdin)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdin`]
    pub stdin: Option<Stdio>,

    /// Configuration for the child process's standard output (stdout)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdout`]
    pub stdout: Option<Stdio>,

    /// Configuration for the child process's standard error (stderr)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stderr`]
    pub stderr: Option<Stdio>,
}

impl Command {
    /// Constructs a new [`Command`] for launching the program at path
    /// `program`. This is just a builder, it does not launch any
    /// program on its own. Only I/O connectors do spawn processes.
    ///
    /// Refs: [`std::process::Command::new`]
    pub fn new<S: Into<OsString>>(program: S) -> Self {
        Self {
            program: program.into(),
            args: None,
            envs: None,
            current_dir: None,
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }

    /// Adds an argument to pass to the program.
    ///
    /// Refs: [`std::process::Command::arg`]
    pub fn arg<S: Into<OsString>>(&mut self, arg: S) -> &mut Self {
        match &mut self.args {
            Some(args) => {
                args.push(arg.into());
            }
            None => {
                self.args = Some(vec![arg.into()]);
            }
        }
        self
    }

    /// Adds multiple arguments to pass to the program.
    ///
    /// Refs: [`std::process::Command::args`]
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    /// Inserts or updates an explicit environment variable mapping.
    ///
    /// Refs: [`std::process::Command::env`]
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: Into<OsString>,
        V: Into<OsString>,
    {
        match &mut self.envs {
            Some(envs) => {
                envs.insert(key.into(), val.into());
            }
            None => {
                self.envs = Some(HashMap::from_iter(Some((key.into(), val.into()))));
            }
        }
        self
    }

    /// Inserts or updates multiple explicit environment variable
    /// mappings.
    ///
    /// Refs: [`std::process::Command::envs`]
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<OsString>,
        V: Into<OsString>,
    {
        for (key, val) in vars {
            self.env(key, val);
        }
        self
    }

    /// Removes an explicitly set environment variable and prevents
    /// inheriting it from a parent process.
    ///
    /// Refs: [`std::process::Command::env_remove`]
    pub fn env_remove<K: Into<OsString>>(&mut self, key: K) -> &mut Self {
        if let Some(envs) = &mut self.envs {
            envs.remove(&key.into());
        }
        self
    }

    /// Clears all explicitly set environment variables and prevents
    /// inheriting any parent process environment variables.
    ///
    /// Refs: [`std::process::Command::env_clear`]
    pub fn env_clear(&mut self) -> &mut Self {
        if let Some(envs) = &mut self.envs {
            envs.clear();
        }
        self.envs = None;
        self
    }

    /// Sets the working directory for the child process.
    ///
    /// Refs: [`std::process::Command::current_dir`]
    pub fn current_dir<P: Into<PathBuf>>(&mut self, dir: P) -> &mut Self {
        self.current_dir = Some(dir.into());
        self
    }

    /// Configuration for the child process's standard input (stdin)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdin`]
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdin = Some(cfg.into());
        self
    }

    /// Configuration for the child process's standard output (stdout)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stdout`]
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdout = Some(cfg.into());
        self
    }

    /// Configuration for the child process's standard error (stderr)
    /// handle.
    ///
    /// Refs: [`std::process::Command::stderr`]
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stderr = Some(cfg.into());
        self
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        let mut command = Command::new(&self.program);

        if let Some(args) = self.args.as_ref() {
            for arg in args {
                command.arg(arg);
            }
        }

        if let Some(envs) = self.envs.as_ref() {
            for (key, val) in envs {
                command.env(key, val);
            }
        }

        if let Some(dir) = self.current_dir.as_ref() {
            command.current_dir(dir);
        }

        command
    }
}

impl Eq for Command {}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        if self.program != other.program {
            return false;
        }

        if self.args != other.args {
            return false;
        }

        if self.current_dir != other.current_dir {
            return false;
        }

        true
    }
}
