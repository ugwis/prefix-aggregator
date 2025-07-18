use std::fmt::{Display, Formatter};
use std::io::{self, BufRead};
use std::net::AddrParseError;
use std::net::Ipv4Addr;
use std::num::ParseIntError;
use std::str::FromStr;

enum MyError {
    ParseIntError(ParseIntError),
    ParseIpv4Error(AddrParseError),
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            MyError::ParseIntError(ref e) => write!(f, "Parse int error {e}"),
            MyError::ParseIpv4Error(ref e) => write!(f, "Parse Ipv4 address error {e}"),
        }
    }
}

impl std::convert::From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::ParseIntError(e)
    }
}

impl std::convert::From<AddrParseError> for MyError {
    fn from(e: AddrParseError) -> Self {
        MyError::ParseIpv4Error(e)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Ipv4Cidr {
    address: Ipv4Addr,
    mask: u8,
}

impl Display for Ipv4Cidr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.address, self.mask)
    }
}

impl Ipv4Cidr {
    fn new(address: Ipv4Addr, mask: u8) -> Self {
        let cidr = Ipv4Cidr { address, mask };
        Self {
            address: cidr.network_addr(),
            mask: cidr.network_mask(),
        }
    }
    fn mask_filter(&self) -> u32 {
        (!0u32).checked_shr(self.mask as u32).unwrap_or(0)
    }
    fn network_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from(u32::from(self.address) & !self.mask_filter())
    }
    fn broadcast_addr(&self) -> Ipv4Addr {
        Ipv4Addr::from(u32::from(self.address) | self.mask_filter())
    }
    fn network_mask(&self) -> u8 {
        self.mask
    }
    fn generate_wrap_cidr(&self) -> Ipv4Cidr {
        Ipv4Cidr::new(self.address, self.mask - 1)
    }
}

impl FromStr for Ipv4Cidr {
    type Err = MyError;
    fn from_str(cidr: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = cidr.split('/').collect();
        Ok(Ipv4Cidr::new(
            Ipv4Addr::from_str(v[0]).unwrap(),
            v[1].parse::<u8>()?,
        ))
    }
}

// Merge two CIDR blocks
fn merge(cidr1: Ipv4Cidr, cidr2: Ipv4Cidr) -> Option<Ipv4Cidr> {
    // Check wether cidr1 includes cidr2 vice versa
    if cidr1.network_addr() <= cidr2.network_addr()
        && cidr2.broadcast_addr() <= cidr1.broadcast_addr()
    {
        return Some(cidr1);
    }
    if cidr2.network_addr() <= cidr1.network_addr()
        && cidr1.broadcast_addr() <= cidr2.broadcast_addr()
    {
        return Some(cidr2);
    }

    // Check wether cidr1 is adjascent to cidr2 vice versa
    let cidr3 = cidr1.generate_wrap_cidr();
    let cidr4 = cidr2.generate_wrap_cidr();
    //println!("[{}-{}]{} == {}: {}", cidr1, cidr2, cidr3, cidr4, cidr3==cidr4);
    if cidr3 == cidr4 && cidr1 != cidr2 {
        return Some(cidr3);
    }
    None
}

// Check wether two CIDR blocks are not mergable but adjascent
fn is_adjascent(cidr1: Ipv4Cidr, cidr2: Ipv4Cidr) -> bool {
    u32::from(cidr1.broadcast_addr()) + 1 == u32::from(cidr2.network_addr())
}

fn main() {
    let stdin = io::stdin();
    let mut cidrs = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.as_str().trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cidr = match Ipv4Cidr::from_str(line) {
            Ok(cidr) => cidr,
            Err(e) => panic!("Failed to parse {:?} as CIDR: {}", line, e),
        };
        cidrs.push(cidr);
    }
    //cidrs.sort();
    /*cidrs.sort_by(|a, b| {
        calc_network_addr(b).cmp(calc_network_addr(&a)));
    }*/
    let mut stack = Vec::new();
    for cidr in cidrs {
        stack.push(cidr);
        //println!("{:?}", stack);
        while stack.len() >= 2 {
            let cidr2 = stack.pop().unwrap();
            let cidr1 = stack.pop().unwrap();
            let merged = merge(cidr1, cidr2);
            match merged {
                Some(merged) => {
                    stack.push(merged);
                }
                None => {
                    stack.push(cidr1);
                    if !is_adjascent(cidr1, cidr2) {
                        for x in stack {
                            println!("{x}");
                        }
                        stack = Vec::new();
                    }
                    stack.push(cidr2);
                    break;
                }
            }
        }
    }
    for x in stack {
        println!("{x}");
    }
}
