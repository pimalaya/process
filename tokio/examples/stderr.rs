use std::io::stderr;

use process_lib::{Command, SpawnCommandThenWaitWithOutput};
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
    command.arg("bad command");
    println!("command: {command:#?}");

    let mut flow = SpawnCommandThenWaitWithOutput::new(command);

    while let Some(io) = flow.next() {
        conn.execute(&mut flow, io).await.unwrap();
    }

    let output = flow.take_output().unwrap();
    println!();
    println!("output: {output:#?}");
}
