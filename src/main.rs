use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Process {
    pid: u32,
}

impl Process {
    pub fn name(&self) -> Result<String, io::Error> {
        let path_string = format!("/proc/{}/cmdline", self.pid); //fix
        let path = Path::new(&path_string);

        match fs::read_to_string(path) {
            Ok(name) => return Ok(name),
            Err(e) => {
                eprintln!("Failed to read /proc/{}/cmdline: {}", self.pid, e); //fix
                return Err(e);
            }
        }
    }

    pub fn memory_regions(&self) {
        let maps_path = String::from(format!("/proc/{}/maps", self.pid));
        let contents = match fs::read_to_string(&maps_path) {
            Err(_) => todo!(),
            Ok(maps) => maps,
        };

        for line in contents.split("\n") {
            for section in line.split_whitespace() {
                println!("{}", section);
            }
        }
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

fn enumerate_processes() -> Option<Vec<Process>> {
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

fn print_all_procs() {
    let procs = enumerate_processes().unwrap_or(Vec::new());

    procs.iter().for_each(|proc| {
        println!(
            "{}: {}",
            proc.pid,
            proc.name().unwrap_or(String::from("<<NONE>>"))
        )
    });
}

fn main() {
    let procs = enumerate_processes().unwrap_or(Vec::new());

    let mut procs = procs.iter();

    let proc: Option<Process> =
        procs.find_map(|proc| match proc.name().unwrap().contains("decrement") {
            true => Some(Process { pid: proc.pid }),
            false => None,
        });
    dbg!(proc.unwrap());
}
