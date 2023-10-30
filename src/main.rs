use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Process {
    pid: u32,
    dir: DirEntry,
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
        let contents = fs::read_to_string(maps_path).expect("Cannot read file");
        println!("{contents}");
    }
}

fn enumerate_processes() -> Option<Vec<Process>> {
    let mut procs: Vec<Process> = Vec::new();
    let path = Path::new("/proc");

    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        if entry.path().is_dir() {
            if entry
                .file_name()
                .into_string()
                .expect("not a interger type path")
                .parse::<u32>()
                .is_ok()
            {
                let proc = Process {
                    pid: entry
                        .file_name()
                        .into_string()
                        .expect("not an interger")
                        .parse::<u32>()
                        .unwrap(),
                    dir: entry,
                };
                procs.push(proc);
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

    for proc in procs.iter() {
        if proc.name().unwrap().contains(&String::from("decrement")) {
            dbg!(proc);
            dbg!(proc.name().unwrap());
        }
    }
}
