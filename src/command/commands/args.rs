use crate::command::parse::CliArgs;
use crate::display::section;

pub fn args(a: CliArgs) {
    let mut args = a;
    let mut opts = String::new();

    if args.has_option("--count") {
        let count = args.options.len() + args.ids.len();
        opts = format!("{}
Count: {}", opts, count);
    }


    let body = format!(
        "
Comm: {:?}
Text: {:?}
Opts: {:?}{}
    ",
        args.command,
        args.ids.join(" | "),
        args.options.join(" | "),
        opts,
    );
    section::section("Arguments", body);
}
