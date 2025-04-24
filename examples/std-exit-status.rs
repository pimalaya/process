#![cfg(feature = "std")]

use std::{env::temp_dir, fs};

use process_flows::{flows::SpawnThenWait, handlers::std::handle, Command};
use uuid::Uuid;

fn main() {
    env_logger::init();

    let mut command = Command::new("touch");
    let path = temp_dir().join(Uuid::new_v4().to_string());
    command.arg(&path);
    println!("spawn: {command:#?}");
    println!();

    let mut spawn = SpawnThenWait::new(command);

    loop {
        match spawn.next() {
            Ok(status) => break println!("exit status: {status:#?}"),
            Err(io) => handle(&mut spawn, io).unwrap(),
        }
    }

    fs::remove_file(path).expect("temp file should be removed");
}
