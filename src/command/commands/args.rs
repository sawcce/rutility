use crate::command::parse::CliArgs;
use crate::display::options::Options;
use crate::display::section;

pub fn args(a: CliArgs) {
    let mut args = a;

    let mut options = Options::new();

    if args.has_option("--count") {
        let count = args.options.len() + args.ids.len();
        options.push(format!("Count: {}", count));
    }

    let body = format!(
"Comm: {:?}
Text: {:?}
Opts: {:?}",
        args.command,
        args.ids.join(" | "),
        args.options.join(" | "),
    );
    section::section("Arguments", body, options);
}
