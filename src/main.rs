use std::fmt;
use std::fs;

use raspvision::Os;
fn main() {
    let os = match Os::build("/etc/os-release") {
        Ok(os) => os,
        Err(e) => {
            eprintln!("Erreur : {}", e);
            return;
        }
    };
    println!("{:?}", os);
}
