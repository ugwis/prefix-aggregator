use std::io::{self, BufRead};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::num::ParseIntError;

struct Ipv4Cidr {
    address: Ipv4Addr,
    mask: u8,
}

impl Ipv4Cidr {
    fn to_string(&self) -> String {
        format!("{}/{}", self.address, self.mask)
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
        Ipv4Cidr{
            address: self.address,
            mask: self.mask - 1,
        }
    }
}

impl FromStr for Ipv4Cidr {
    type Err = ParseIntError;
    fn from_str(cidr: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = cidr.split('/').collect();
        let address = Ipv4Addr::from_str(v[0]).unwrap();
        //self.addresss = self.network_addr();
        let mask = v[1].parse::<u8>().unwrap();
        Ok(Ipv4Cidr{
            address: address,
            mask: mask,
        })
    }
}

/*fn ipv4todec(ipv4: String) -> u32 {
    let v: Vec<&str> = ipv4.split('.').collect();
    let mut sum: u32 = 0;
    for x in v {
        sum = sum*256 + x.parse::<u32>().unwrap();
    }
    return sum;
}

fn dectoipv4(mut dec: u32) -> String {
    let mut ret: String = "".to_string();
    for i in 0..4 {
        if i > 0 {
            ret = ".".to_string() + &ret;
        }
        ret = (dec % 256).to_string() + &ret;
        dec /= 256;
    }
    return ret;
}*/

// Merge two CIDR blocks
fn merge(cidr1: Ipv4Cidr, cidr2: Ipv4Cidr) -> Vec<Ipv4Cidr> {
    // Check wether cidr1 includes cidr2 vice versa
    if cidr1.network_addr() <= cidr2.network_addr() && cidr2.broadcast_addr() <= cidr1.broadcast_addr() {
        return vec![cidr1];
    }
    if cidr2.network_addr() <= cidr1.network_addr() && cidr1.broadcast_addr() <= cidr2.broadcast_addr() {
        return vec![cidr2];
    }

    // Check wether cidr1 is adjascent to cidr2 vice versa
    if cidr1.network_mask() == cidr2.network_mask() {
        let cidr3 = cidr1.generate_wrap_cidr();
        if cidr1.network_addr() < cidr2.network_addr() {
            if cidr1.network_addr() == cidr3.network_addr() && cidr2.broadcast_addr() == cidr3.broadcast_addr() {
                return vec![cidr3];
            }
        }
        if cidr2.network_addr() < cidr1.network_addr() {
            if cidr2.network_addr() == cidr3.network_addr() && cidr1.broadcast_addr() == cidr3.broadcast_addr() {
                return vec![cidr3];
            }
        }
    }
    return vec![cidr1, cidr2];
}

// Check wether two CIDR blocks are adjascent
/*fn is_adjascent(cidr1: String, cidr2: String) -> bool {
    let cidr2_net = ipv4todec(calc_network_addr(cidr2.clone()));
    let cidr1_brd = ipv4todec(calc_broadcast_addr(cidr1.clone()));
    return cidr1_brd + 1 == cidr2_net;
}*/

fn main() {
    let stdin = io::stdin();
    let mut cidrs = Vec::new();
    for line in stdin.lock().lines() {
        cidrs.push(Ipv4Cidr::from_str(&line.unwrap()));
    }
    //cidrs.sort();
    /*cidrs.sort_by(|a, b| {
        calc_network_addr(b).cmp(calc_network_addr(&a)));
    }*/
    /*let mut stack = Vec::new();
    for cidr in cidrs {
        stack.push(cidr);
        //println!("{:?}", stack);
        while stack.len() >= 2 {
            let m2 = stack.pop().unwrap();
            let m1 = stack.pop().unwrap();
            let merged = merge(m1.clone(), m2.clone());
            if merged.len() == 1 {
                //mergeable
                stack.push(merged[0].clone());
            } else {
                stack.push(merged[0].clone());
                if ! is_adjascent(m1.clone(), m2.clone()) {
                    for x in stack {
                        println!("{}", x);
                    }
                    stack = Vec::new();
                }
                stack.push(merged[1].clone());
                break;
            }
        }
    }
    for x in stack {
        println!("{}", x);
    }*/
    /*let args: Vec<String> = env::args().collect();
    let dec = dectoipv4(ipv4todec(&args[1]));
    println!("{}", dec);*/
}
