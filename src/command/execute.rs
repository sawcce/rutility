use crate::parse::CliArgs;

use crate::command::commands;

pub fn execute(args: CliArgs) {
    println!("{}", args.command);

    match &args.command as &str {
        "info" => {
            commands::info::info();
        }
        "args" => {
            commands::args::args(args);
        }
        _ => {
            println!("Unknown command: {}", args.command)
        }
    }
}
