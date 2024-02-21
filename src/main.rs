use sysinfo::{
    Components, CpuRefreshKind, Disks, Networks, Process, RefreshKind, System, MINIMUM_CPU_UPDATE_INTERVAL
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
        writeln!(f, "Mémoire utilisée : {:}/{:} bytes   ", self.used, self.total)
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

    let mut s = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    loop
    {
        std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL * 50);
        s.refresh_cpu();
        for cpu in s.cpus()
        {
            println!("{}%", cpu.cpu_usage());
        }
    }

}
