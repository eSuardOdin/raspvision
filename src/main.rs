use sysinfo::{
    Components, Disks, Networks, System
};
use std::fmt;


fn bytes_to_gb(bytes: u64) -> f64
{
    bytes as f64 / (1024i32.pow(3) as f64)
}


// Structure perso
struct Ram
{
    used: u64,
    free: u64,
    total: u64,
}

impl fmt::Display for Ram
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // writeln!(f, "Mémoire utilisée : {:.2}/{:.2} Gb", bytes_to_gb(self.used), bytes_to_gb(self.total))
        writeln!(f, "Mémoire utilisée : {:}/{:} Gb", self.used, self.total)
    }
}

fn main() {
    let mut sys = System::new_all(); // Permet de tout init
    sys.refresh_all();

    let mut ram = Ram {
        used: sys.used_memory(),
        free: sys.free_memory(),
        total: sys.total_memory()
    };
    println!("{}", ram);
}
