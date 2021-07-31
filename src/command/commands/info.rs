use crate::display::section;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn info() {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error");

    section::section(
        "Info",
        format!(
            "
Version: 0.1,
Time: {:?} (unix)
    ",
            time.as_millis()
        ),
    )
}
