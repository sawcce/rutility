use crate::display::options::Options;

pub fn section(title: &str, content: String, options: Options) {
    println!("--- {} ---", title);
    println!("{}", content);
    println!("{}", options.display());
    println!("------");
}