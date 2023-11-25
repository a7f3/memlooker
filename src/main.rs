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

fn get_target_from_list(proc: &Process, list: &Vec<Address>, target: u32) -> Vec<Address> {
    let mut addr_list = Vec::<Address>::new();
    for region in proc.get_all_memory_regions() {
        for addr in list {
            if !region.in_region(addr) {
                continue;
            }

            match region.read_addr(addr) {
                Some(a) => {
                    if a == target {
                        addr_list.push(addr.clone())
                    }
                }
                None => (),
            }
        }
    }

    return addr_list;
}

fn main() {
    let proc = match get_proc_by_name("decrement") {
        None => panic!("Unable to find process!"),
        Some(p) => p,
    };

    println!("{:?}: {:?}", proc.pid, proc.name());

    let mut addr_list: Vec<Address> = Vec::<Address>::new();
    let mut old_addr_list: Vec<Address> = Vec::<Address>::new();

    loop {
        println!("Please input a number: ");
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);

        let target: u32 = match line.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                print!("{:?}", addr_list);
                println!("Error: {}", e);
                continue;
            }
        };

        /* run this once to get the lists started */
        if old_addr_list.len() == 0 {
            addr_list = get_target(&proc, target);
            continue;
        }

        old_addr_list = addr_list.clone();
        addr_list = get_target_from_list(&proc, &old_addr_list, target);
        addr_list = addr_list
            .clone()
            .into_iter()
            .filter(|item| old_addr_list.contains(item))
            .collect::<Vec<Address>>();
        dbg!(&addr_list);
    }
}
