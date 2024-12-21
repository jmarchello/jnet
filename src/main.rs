mod commands;

use std::env;

fn main() {
    let command = env::args().nth(1);
    match command {
        None => println!("Please provide a subcommand"),
        Some(cmd) => run_command(cmd)
    };
}

fn run_command(subcommand: String) {
    match subcommand.as_str() {
        "port" => commands::scan_port(),
        _ => println!("huh?")
    };
}
