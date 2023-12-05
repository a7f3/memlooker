use crate::process::address;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, SeekFrom};

use super::address::Address;

#[derive(Debug)]
struct Perms {
    read: bool,
    write: bool,
    execute: bool,
    shared: bool,
    private: bool, /* copy on write*/
}

#[derive(Debug)]
pub struct MemoryRegion {
    addr_range: address::Range,
    perms: Perms,
    offset: address::Address,
    pub pathname: String,
}

impl fmt::Display for Perms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.read { "r" } else { "-" },
            if self.write { "w" } else { "-" },
            if self.execute { "x" } else { "-" },
            if self.private {
                "p"
            } else if self.shared {
                "s"
            } else {
                "-"
            }
        )
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
            if c == 'x' {
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

impl fmt::Display for MemoryRegion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.addr_range, self.perms, self.offset, self.pathname
        )
    }
}

impl MemoryRegion {
    pub fn new_from_str(line: &str) -> Option<Self> {
        let mut split_line = line.split_whitespace();

        let addr_range = address::Range::new_from_str(split_line.next()?);

        /* Parses the second split into a Perms */
        let perms = match split_line.next() {
            None => return None,
            Some(p) => Perms::new_from_str(p),
        };

        let offset = address::Address::new_from_str(split_line.next()?)?;

        /* skip to the last element */
        split_line.next();
        split_line.next();

        let pathname = match split_line.next() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };

        Some(MemoryRegion {
            addr_range,
            perms,
            offset,
            pathname,
        })
    }

    fn read_4_bytes(&self, f: &mut File, offset: u64, buffer: &mut [u8]) -> u64 {
        let _ = f.read_exact(buffer);
        let _ = f.seek(SeekFrom::Start(offset));
        offset + buffer.len() as u64
    }

    pub fn as_u32_be(array: &[u8; 4]) -> u32 {
        ((array[0] as u32) << 24)
            + ((array[1] as u32) << 16)
            + ((array[2] as u32) << 8)
            + (array[3] as u32)
    }

    fn as_u32_le(array: &[u8; 4]) -> u32 {
        (array[0] as u32)
            + ((array[1] as u32) << 8)
            + ((array[2] as u32) << 16)
            + ((array[3] as u32) << 24)
    }

    fn as_u32(array: &[u8; 4]) -> u32 {
        if cfg!(target_endian = "big") {
            Self::as_u32_be(array)
        } else {
            Self::as_u32_le(array)
        }
    }
    pub fn read_mem_from_addr_list(
        &self,
        pid: u32,
        addrs: &Vec<Address>,
        target: u32,
    ) -> Option<Vec<Address>> {
        if !self.perms.read {
            return None;
        }

        let mut f = match File::open(format!("/proc/{}/mem", pid)) {
            Ok(f) => f,
            Err(e) => panic!("cannot open /proc/{pid}/mem: {e}"),
        };

        let mut addr_list: Vec<Address> = Vec::<Address>::new();
        for addr in addrs {
            if addr.addr < self.addr_range.start.addr || addr.addr > self.addr_range.end.addr {
                continue;
            }
            let mut buffer = [0_u8; 4];

            let _ = self.read_4_bytes(&mut f, addr.addr, &mut buffer);

            let num = Self::as_u32(&buffer);
            if num == target {
                addr_list.push(addr.clone());
            }
        }
        Some(addr_list)
    }

    pub fn read_mem(&self, pid: u32, target: u32) -> Option<Vec<Address>> {
        if !self.perms.read {
            return None;
        }

        let mut f = match File::open(format!("/proc/{}/mem", pid)) {
            Ok(f) => f,
            Err(e) => panic!("cannot open /proc/{pid}/mem: {e}"),
        };

        let mut offset = self.addr_range.start.addr;
        let mut buffer = [0_u8; 4];
        let mut addr_list: Vec<Address> = Vec::<Address>::new();

        while offset < self.addr_range.end.addr {
            offset = self.read_4_bytes(&mut f, offset, &mut buffer);

            let num = Self::as_u32(&buffer);
            if num == target {
                addr_list.push(Address { addr: offset });
            }
        }

        Some(addr_list)
    }

    // Tests if addr is in the region, start and end inclusive
    pub fn in_region(&self, addr: &Address) -> bool {
        self.addr_range.start.addr <= addr.addr && addr.addr <= self.addr_range.end.addr
    }

    /// Reads value from address and returns it
    pub fn read_addr(&self, addr: &Address) -> Option<u32> {
        if !self.perms.read {
            return None;
        }

        let mut f = match File::open(format!("{}/mem", self.pathname)) {
            Ok(f) => f,
            Err(e) => panic!("cannot open {}/mem: {e}", self.pathname),
        };

        let mut buffer = [0_u8; 4];
        self.read_4_bytes(&mut f, addr.addr, &mut buffer);
        Some(Self::as_u32(&buffer))
    }
}

#[cfg(test)]
mod tests {
    mod in_region {
        use super::super::*;
        fn setup() -> MemoryRegion {
            MemoryRegion {
                addr_range: address::Range {
                    start: Address { addr: 1 },
                    end: Address { addr: 100 },
                },
                perms: Perms {
                    read: true,
                    write: false,
                    execute: false,
                    shared: false,
                    private: false,
                },
                offset: Address { addr: 123 },
                pathname: "/proc/0".to_string(),
            }
        }
        #[test]
        fn start_inclusive() {
            let region = setup();
            let addr = Address { addr: 1 };
            assert!(region.in_region(&addr));
        }

        #[test]
        fn end_inclusive() {
            let region = setup();
            let addr = Address { addr: 100 };
            assert!(region.in_region(&addr));
        }

        #[test]
        fn inside() {
            let region = setup();
            let addr = Address { addr: 50 };
            assert!(region.in_region(&addr));
        }

        #[test]
        fn out_above_range() {
            let region = setup();
            let addr = Address { addr: 101 };
            assert!(!region.in_region(&addr));
        }

        #[test]
        fn outside_below_range() {
            let region = setup();
            let addr = Address { addr: 0 };
            assert!(!region.in_region(&addr));
        }
    }
}
