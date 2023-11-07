use std::fs;
use std::path::Path;

use crate::process::Process;

mod process;

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

    for region in proc.get_all_memory_regions() {
        println!("{}", region);
        region.read_mem(proc.pid);
    }
}
