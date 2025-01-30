use std::{env::temp_dir, fs, io::stderr};

use process_lib::{Command, SpawnCommandThenWait};
use process_std::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let conn = Connector::new();

    let mut command = Command::new("touch");
    let path = temp_dir().join(Uuid::new_v4().to_string());
    command.arg(&path);
    println!("command: {command:#?}");

    let mut flow = SpawnCommandThenWait::new(command);

    while let Some(io) = flow.next() {
        conn.execute(&mut flow, io).unwrap();
    }

    let status = flow.take_status().unwrap();
    println!();
    println!("status: {status:#?}");

    fs::remove_file(path).expect("temp file should exist");
}
