#![cfg(feature = "tokio")]

use std::process::Stdio;

use process_flows::{
    flows::{SpawnThenWait, SpawnThenWaitWithOutput},
    handlers::tokio::handle,
    Command,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut command = Command::new("/bin/sh");
    command.arg("-c");
    command.arg("read line; echo $line");
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::piped());
    println!("spawn 1: {command:#?}");
    println!();
    println!("What is your name? ");

    let mut spawn = SpawnThenWait::new(command);

    let (status, stdout) = loop {
        match spawn.next() {
            Ok(status) => break (status, spawn.take_stdout().unwrap()),
            Err(io) => handle(&mut spawn, io).await.unwrap(),
        }
    };

    println!();
    println!("status: {status:#?}");
    println!();

    let mut command = Command::new("cat");
    command.arg("-E");
    command.stdin(stdout);
    println!("command 2: {command:#?}");
    println!();

    let mut spawn = SpawnThenWaitWithOutput::new(command);

    loop {
        match spawn.next() {
            Ok(output) => break println!("output: {output:#?}"),
            Err(io) => handle(&mut spawn, io).await.unwrap(),
        }
    }
}
