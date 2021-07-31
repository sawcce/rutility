#[derive(Debug)]
pub struct CliArgs {
    pub command: String,
    pub ids: Vec<String>,
    pub options: Vec<String>,
}

impl CliArgs {
    fn new(command: String) -> CliArgs {
        return CliArgs {command, ids: Vec::new(), options: Vec::new()};
    }

    fn add_id(&mut self, id: String){
        self.ids.push(id);
    }

    fn add_option(&mut self, option: String){
        self.options.push(option);
    }

    pub fn has_option(&mut self, option: &str) -> bool{
        return self.options.contains(&String::from(option));
    }
}

pub fn parse() -> CliArgs {
    let mut args: Vec<String> = std::env::args().collect();
    let _ = args.remove(0);
    if args.len() >= 1 {

        let command = args.remove(0);

        let mut fin = CliArgs::new(command);
    
        for argument in args {
            if argument.starts_with("--") {
                fin.add_option(argument);
            }else {
                fin.add_id(argument);
            }
        }
    
        return fin;
    }

    return CliArgs::new(String::from("info"));
}