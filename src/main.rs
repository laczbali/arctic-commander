mod utils;
use utils::console::console_clear;
use utils::filesystem::list_dir;

use std::path::Path;

fn main() {
    console_clear();

    let contents = list_dir(Path::new("."));
    for c in contents {
        println!("{}", c.display());
    }
}
