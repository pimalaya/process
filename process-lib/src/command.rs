use std::{collections::HashMap, ffi::OsString, path::PathBuf, process::Stdio};

#[derive(Debug)]
pub struct Command {
    pub program: OsString,
    pub args: Option<Vec<OsString>>,
    pub envs: Option<HashMap<OsString, OsString>>,
    pub current_dir: Option<PathBuf>,
    pub stdin: Option<Stdio>,
    pub stdout: Option<Stdio>,
    pub stderr: Option<Stdio>,
}

impl Command {
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

    pub fn env_remove<K: Into<OsString>>(&mut self, key: K) -> &mut Self {
        if let Some(envs) = &mut self.envs {
            envs.remove(&key.into());
        }
        self
    }

    pub fn env_clear(&mut self) -> &mut Self {
        if let Some(envs) = &mut self.envs {
            envs.clear();
        }
        self.envs = None;
        self
    }

    pub fn current_dir<P: Into<PathBuf>>(&mut self, dir: P) -> &mut Self {
        self.current_dir = Some(dir.into());
        self
    }

    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdin = Some(cfg.into());
        self
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stdout = Some(cfg.into());
        self
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        self.stderr = Some(cfg.into());
        self
    }
}
