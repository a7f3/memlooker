pub mod address;
mod memory_region;
use memory_region::MemoryRegion;

use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Process {
    pub pid: u32,
}

impl Process {
    pub fn name(&self) -> Result<String, io::Error> {
        let path_string = format!("/proc/{}/cmdline", self.pid);
        let path = Path::new(&path_string);

        match fs::read_to_string(path) {
            Ok(name) => Ok(name),
            Err(e) => {
                eprintln!("Failed to read /proc/{}/cmdline: {}", self.pid, e);
                Err(e)
            }
        }
    }

    pub fn get_all_memory_regions(&self) -> Vec<MemoryRegion> {
        let maps_path = format!("/proc/{}/maps", self.pid);
        let contents = match fs::read_to_string(maps_path) {
            Err(e) => panic!("ERROR {}: Cannot access /proc/{}/maps", e, self.pid),
            Ok(maps) => maps,
        };

        let mut regions: Vec<MemoryRegion> = Vec::<MemoryRegion>::new();

        for line in contents.split('\n') {
            let region = match MemoryRegion::new_from_str(line) {
                Some(region) => region,
                None => continue,
            };

            regions.push(region);
        }
        println!("{} regions", regions.len());
        regions
    }
}

impl Process {
    pub fn new(pid: u32) -> Option<Self> {
        if pid == 0 {
            return None;
        }
        if Path::new(&format!("/proc/{}/", pid)).exists() {
            Some(Self { pid })
        } else {
            None
        }
    }
}
