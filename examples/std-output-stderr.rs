#![cfg(feature = "std")]

use process_flows::{flows::SpawnThenWaitWithOutput, handlers::std::handle, Command};

fn main() {
    env_logger::init();

    let mut command = Command::new("/bin/sh");
    command.arg("-c");
    command.arg("bad command");
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
