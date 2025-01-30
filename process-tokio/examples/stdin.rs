use std::{io::stderr, process::Stdio};

use process_lib::{Command, SpawnCommandThenWait, SpawnCommandThenWaitWithOutput};
use process_tokio::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let conn = Connector::new();

    let mut command = Command::new("/bin/sh");
    command.arg("-c");
    command.arg("read line; echo $line");
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::piped());
    println!("command 1: {command:#?}");
    println!();
    println!("What is your name? ");

    let mut flow = SpawnCommandThenWait::new(command);

    while let Some(io) = flow.next() {
        conn.execute(&mut flow, io).await.unwrap();
    }

    let stdout = flow.take_stdout().unwrap();
    let status = flow.take_status().unwrap();
    println!();
    println!("status: {status:#?}");

    let mut command = Command::new("cat");
    command.arg("-E");
    command.stdin(stdout);
    println!();
    println!("command 2: {command:#?}");

    let mut flow = SpawnCommandThenWaitWithOutput::new(command);

    while let Some(io) = flow.next() {
        conn.execute(&mut flow, io).await.unwrap();
    }

    let output = flow.take_output().unwrap();
    println!();
    println!("output: {output:#?}");
}
