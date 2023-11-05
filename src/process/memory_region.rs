use crate::Process::address::{Address, AddressRange};

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
    addr_range: AddressRange,
    perms: Perms,
    offset: Address,
    pathname: String,
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
    pub fn new_from_str(line: &str) -> Option<Self> {
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
