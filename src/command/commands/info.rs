use crate::display::section;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::display::options::Options;

pub fn info() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error");

    let options = Options::new();

    section::section(
        "Info",
        format!(
            "
Version: 0.1,
Time: {:?} (unix)
    ",
            time.as_millis()
        ),
        options,
    )
}
