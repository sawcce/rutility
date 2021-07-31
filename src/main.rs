mod command;
mod display;

use command::parse;
use command::execute;

fn main() {
    let arguments = parse::parse();
    execute::execute(arguments);
}
