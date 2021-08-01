use crate::display::options::Options;

/// Function to display in the console a section like
/// --- Title ---
/// ..content
/// ...options
/// ------
pub fn section(title: &str, content: String, options: Options) {
    println!("--- {} ---", title);
    println!("{}", content);
    if options.content.len() > 0 {
        println!("{}", options.display());
    }
    println!("------");
}