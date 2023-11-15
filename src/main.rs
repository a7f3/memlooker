use std::fs;
use std::io;
use std::path::Path;

use crate::process::address::Address;
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

fn get_target(proc: &Process, target: u32) -> Vec<Address> {
    let mut addr_list = Vec::<Address>::new();
    for region in proc.get_all_memory_regions() {
        match region.read_mem(proc.pid, target) {
            Some(a) => {
                let mut b = a;
                addr_list.append(&mut b)
            }
            None => (),
        }
    }

    return addr_list;
}

fn main() {
    let proc = match get_proc_by_name("decrement") {
        None => panic!("Unable to find process!"),
        Some(p) => p,
    };

    let mut addr_list: Vec<Address> = Vec::<Address>::new();
    let mut old_addr_list: Vec<Address> = Vec::<Address>::new();

    loop {
        println!("Please input a number: ");
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);

        let target: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        /* run this once to get the lists started */
        if old_addr_list.len() == 0 {
            addr_list = get_target(&proc, target);
        }

        old_addr_list = addr_list.clone();
        addr_list = get_target(&proc, target);
        addr_list = addr_list
            .clone()
            .into_iter()
            .filter(|item| old_addr_list.contains(item))
            .collect::<Vec<Address>>();
        dbg!(&addr_list);
    }
}
