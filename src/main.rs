mod argsplit;

use std::{process::Command, time::Duration};

use clap::Parser;

use crate::argsplit::split_args;

/// A tool to sleep and then possibly execute a command.
#[derive(Parser, Debug)]
struct Args {
    /// Time in seconds to sleep
    #[clap(short, long)]
    sleep: u64,

    /// Command to execute
    #[clap(short, long)]
    command: Option<String>,
}

fn main() {
    // Handle /?
    let args: Vec<_> = std::env::args().collect();
    if args.contains(&"/?".to_owned()) || args.contains(&"-?".to_owned()) {
        Args::parse_from(&["nap.exe", "--help"]);
        std::process::exit(0);
    }

    let args = Args::parse();

    // Sleep
    println!("Sleeping for {} seconds...", args.sleep);
    std::thread::sleep(Duration::from_secs(args.sleep));

    // Execute a command if provided
    if let Some(command_input) = args.command {
        let mut command_args = split_args(&command_input);

        let command_target = command_args.remove(0);
        let mut command = Command::new(&command_target);
        if !command_input.is_empty() {
            command.args(&command_args);
        }
        let mut process = command
            .spawn()
            .expect(&format!("Failed to start process \"{}\"!", command_target));
        process
            .wait()
            .expect("Failed to wait for spawned process to exit!");
    }
}
