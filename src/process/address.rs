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
            Err(_) => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negitive() {
        match Address::new_from_str("-1234567890abcdef") {
            Some(_) => panic!(),
            None => 0,
        };
    }

    #[test]
    fn positive() {
        match Address::new_from_str("1234567890abcdef") {
            Some(a) => assert_eq!(a.addr, 0x1234567890abcdef),
            None => panic!(),
        };
    }

    #[test]
    fn non_hex() {
        match Address::new_from_str("1234567890abcdefg") {
            Some(_) => panic!(),
            None => 0,
        };
    }

    #[test]
    fn too_big() {
        let max_plus_one = format!("{:0x}1", u64::max_value());

        match Address::new_from_str(&max_plus_one) {
            Some(_) => panic!(),
            None => 0,
        };
    }
}
