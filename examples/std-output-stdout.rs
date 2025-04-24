#![cfg(feature = "std")]

use process_flows::{flows::SpawnThenWaitWithOutput, handlers::std::handle, Command};

fn main() {
    env_logger::init();

    let mut command = Command::new("echo");
    command.arg("hello");
    command.arg("world");
    println!("spawn: {command:#?}");
    println!();

    let mut spawn = SpawnThenWaitWithOutput::new(command);
    loop {
        match spawn.next() {
            Ok(output) => break println!("output: {output:#?}"),
            Err(io) => handle(&mut spawn, io).unwrap(),
        }
    }
}
