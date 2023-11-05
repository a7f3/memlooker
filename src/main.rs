use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
struct Process {
    pid: u32,
}

#[derive(Debug)]
struct Perms {
    read: bool,
    write: bool,
    execute: bool,
    shared: bool,
    private: bool, /* copy on write*/
}

#[derive(Debug)]
struct Address {
    addr: u64,
}

impl Address {
    fn new_from_str(addr_str: &str) -> Option<Self> {
        return match u64::from_str_radix(addr_str, 16) {
            Err(e) => panic!("Could not parse int: {}", e),
            Ok(addr) => Some(Address { addr }),
        };
    }
}

#[derive(Debug)]
struct AddressRange {
    start: Address,
    end: Address,
}

#[derive(Debug)]
struct MemoryRegion {
    addr_range: AddressRange,
    perms: Perms,
    offset: Address,
    pathname: String,
}

impl AddressRange {
    fn new_from_str(s: &str) -> Self {
        /* Parses the first split into a AddressRange*/
        let mut a = s.split("-");

        let start = match a.next() {
            None => panic!("cannot make addr from str"),
            Some(hex_str) => match Address::new_from_str(hex_str) {
                Some(a) => a,
                None => panic!("cannot make addr from str"),
            },
        };

        let end = match a.next() {
            None => panic!("cannot make addr from str"),
            Some(hex_str) => match Address::new_from_str(hex_str) {
                Some(a) => a,
                None => panic!("cannot make addr from str"),
            },
        };

        AddressRange { start, end }
    }
}

impl Perms {
    fn new_from_str(perms: &str) -> Self {
        if perms.len() != 4 {
            panic!("Length of perms not correct");
        }

        let mut read: bool = false;
        let mut write: bool = false;
        let mut execute: bool = false;
        let mut private: bool = false;
        let mut shared: bool = false;

        for c in perms.chars() {
            if c == 'r' {
                read = true;
            }
            if c == 'w' {
                write = true;
            }
            if c == 'e' {
                execute = true;
            }
            if c == 'p' {
                private = true;
            }
            if c == 's' {
                shared = true;
            }
        }

        Perms {
            read,
            write,
            execute,
            shared,
            private,
        }
    }
}

impl MemoryRegion {
    fn new_from_str(line: &str) -> Option<Self> {
        let mut split_line = line.split_whitespace();

        let addr_range = AddressRange::new_from_str(split_line.next()?);

        /* Parses the second split into a Perms */
        let perms = match split_line.next() {
            None => return None,
            Some(p) => Perms::new_from_str(p),
        };

        let offset = Address::new_from_str(split_line.next()?)?;
        let pathname = split_line.last()?.to_string();

        return Some(MemoryRegion {
            addr_range,
            perms,
            offset,
            pathname,
        });
    }
}

impl Process {
    pub fn name(&self) -> Result<String, io::Error> {
        let path_string = format!("/proc/{}/cmdline", self.pid);
        let path = Path::new(&path_string);

        match fs::read_to_string(path) {
            Ok(name) => return Ok(name),
            Err(e) => {
                eprintln!("Failed to read /proc/{}/cmdline: {}", self.pid, e);
                return Err(e);
            }
        }
    }

    pub fn get_all_memory_regions(&self) -> Vec<MemoryRegion> {
        let maps_path = String::from(format!("/proc/{}/maps", self.pid));
        let contents = match fs::read_to_string(&maps_path) {
            Err(e) => panic!("ERROR {}: Cannot access /proc/{}/maps", e, self.pid),
            Ok(maps) => maps,
        };

        let mut regions: Vec<MemoryRegion> = Vec::<MemoryRegion>::new();

        for line in contents.split("\n").into_iter() {
            let region = match MemoryRegion::new_from_str(line) {
                Some(region) => region,
                None => continue,
            };

            regions.push(region);
        }
        return regions;
    }
}

impl Process {
    pub fn new(pid: u32) -> Option<Self> {
        if pid == 0 {
            return None;
        }
        if Path::new(&format!("/proc/{}/", pid)).exists() {
            return Some(Self { pid });
        } else {
            return None;
        }
    }
}

fn get_all_pids() -> Option<Vec<Process>> {
    let mut procs: Vec<Process> = Vec::new();
    let path = Path::new("/proc");

    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        if entry.path().is_dir() {
            let pid: Option<u32> = match entry
                .file_name()
                .into_string()
                .expect("not a interger type path")
                .parse::<u32>()
            {
                Ok(pid) => Some(pid),
                Err(_) => None,
            };
            match pid {
                None => continue,
                Some(pid) => {
                    let proc = Process::new(pid);
                    match proc {
                        Some(proc) => procs.push(proc),
                        None => continue,
                    }
                }
            }
        }
    }

    if procs.len() > 0 {
        return Some(procs);
    }
    return None;
}

fn get_proc_by_name(name: &str) -> Option<Process> {
    let procs = get_all_pids().unwrap_or(Vec::new());
    let mut procs = procs.iter();
    procs.find_map(|proc| match proc.name().unwrap().contains(name) {
        true => Some(Process { pid: proc.pid }),
        false => None,
    })
}

fn main() {
    let proc = match get_proc_by_name("decrement") {
        None => panic!("Unable to find process!"),
        Some(p) => p,
    };

    dbg!(proc.get_all_memory_regions());
}
