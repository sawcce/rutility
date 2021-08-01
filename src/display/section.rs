use crate::display::options::Options;

/// Function to display a section in the console
/// 
/// # Example
/// 
/// ```
/// use crate::display::options::Options;
/// let options = Options::new();
/// section("Info", "V.01", options);
/// 
/// ## Gives:
/// 
/// ```
/// 
/// --- Info ---
/// 
/// V.01
pub fn section(title: &str, content: String, options: Options) {
    println!("--- {} ---", title);
    println!("{}", content);
    if options.content.len() > 0 {
        println!("{}", options.display());
    }
    println!("------");
}