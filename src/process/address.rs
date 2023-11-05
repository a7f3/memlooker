#[derive(Debug)]
pub struct Address {
    addr: u64,
}

#[derive(Debug)]
pub struct AddressRange {
    start: Address,
    end: Address,
}

impl Address {
    pub fn new_from_str(addr_str: &str) -> Option<Self> {
        return match u64::from_str_radix(addr_str, 16) {
            Err(e) => panic!("Could not parse int: {}", e),
            Ok(addr) => Some(Address { addr }),
        };
    }
}

impl AddressRange {
    pub fn new_from_str(s: &str) -> Self {
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
