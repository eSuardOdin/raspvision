use std::fmt;
use std::fs;

use raspvision::Os;
fn main() {
    let os = Os::build("/etc/os-release").unwrap();
    println!("{:?}", os);
}
