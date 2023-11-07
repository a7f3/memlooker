use crate::process::address;
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};

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
    pathname: String,
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
            } else {
                if self.shared {
                    "s"
                } else {
                    "-"
                }
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

        return Some(MemoryRegion {
            addr_range,
            perms,
            offset,
            pathname,
        });
    }

    fn read_4_bytes(&self, f: &mut File, offset: u64, buffer: &mut [u8]) -> u64 {
        let _ = f.read_exact(buffer);
        let _ = f.seek(SeekFrom::Start(offset));
        return offset + buffer.len() as u64;
    }

    pub fn as_u32_be(array: &[u8; 4]) -> u32 {
        ((array[0] as u32) << 24)
            + ((array[1] as u32) << 16)
            + ((array[2] as u32) << 8)
            + ((array[3] as u32) << 0)
    }

    pub fn read_mem(&self, pid: u32) {
        if !self.perms.read {
            return;
        }

        let mut f = match File::open(format!("/proc/{}/mem", pid)) {
            Ok(f) => f,
            Err(e) => panic!("cannot open /proc/{pid}/mem: {e}"),
        };

        let mut offset = self.addr_range.start.addr;
        let mut buffer = [0_u8; 4];
        let mut a = 0;

        /*self.read_4_bytes(&mut f, 94773577019228_u64, &mut buffer);
        println!("{}", Self::as_u32_be(&buffer));*/

        while offset < self.addr_range.end.addr {
            offset = self.read_4_bytes(&mut f, offset, &mut buffer);

            let num = Self::as_u32_be(&buffer);
            if num == 51 {
                println!("{}: {}", offset, num);
            }

            /* for i in buffer {
                if i == 0 {
                    print!("..");
                } else {
                    print!("{:02x}", i);
                }
            }*/
            a += 1;
        }
    }
}
