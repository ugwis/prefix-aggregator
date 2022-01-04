use std::io::{self, BufRead};


fn ipv4todec(ipv4: String) -> u32 {
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
}

// Calculate Network address from CIDR block
fn calc_network_addr(cidr: String) -> String {
    let v: Vec<&str> = cidr.split('/').collect();
    let net = v[0].to_string();
    let mask = v[1].parse::<u32>().unwrap();
    let mask_num = 2_u32.pow(32 - mask);
    let network_address_num = ipv4todec(net)/mask_num * mask_num;
    return dectoipv4(network_address_num);
}

// Calculate Broadcast address from CIDR block
fn calc_broadcast_addr(cidr: String) -> String {
    let v: Vec<&str> = cidr.split('/').collect();
    let net = v[0].to_string();
    let mask = v[1].parse::<u32>().unwrap();
    let mask_num = 2_u32.pow(32 - mask);
    let network_address_num = (1 + ipv4todec(net)/mask_num) * mask_num - 1;
    return dectoipv4(network_address_num);
}

// Calculate Network mask from CIDR block
fn calc_network_mask(cidr: String) -> u32 {
    let v: Vec<&str> = cidr.split('/').collect();
    let mask = v[1].parse::<u32>().unwrap();
    return mask;
}

// Generate CIDR from Network address and mask
fn gen_cidr(network: String, mask: u32) -> String {
    return network + "/" + &mask.to_string()
}

// Merge two CIDR blocks
fn merge(cidr1: String, cidr2: String) -> Vec<String> {
    let cidr1_net = ipv4todec(calc_network_addr(cidr1.clone()));
    let cidr2_net = ipv4todec(calc_network_addr(cidr2.clone()));
    let cidr1_brd = ipv4todec(calc_broadcast_addr(cidr1.clone()));
    let cidr2_brd = ipv4todec(calc_broadcast_addr(cidr2.clone()));
    let cidr1_mask = calc_network_mask(cidr1.clone());
    let cidr2_mask = calc_network_mask(cidr2.clone());

    //inclusion
    if cidr1_net <= cidr2_net && cidr2_brd <= cidr1_brd {
        return vec![cidr1];
    }
    if cidr2_net <= cidr1_net && cidr1_brd <= cidr2_brd {
        return vec![cidr2];
    }

    //adjascent
    let cidr1_wrap_net = ipv4todec(calc_network_addr(gen_cidr(dectoipv4(cidr1_net), cidr1_mask - 1)));
    let cidr2_wrap_brd = ipv4todec(calc_broadcast_addr(gen_cidr(dectoipv4(cidr2_net), cidr2_mask - 1)));
    if cidr1_wrap_net == cidr1_net && cidr1_brd + 1 == cidr2_net && cidr2_wrap_brd == cidr2_brd { 
       return vec![gen_cidr(dectoipv4(cidr1_net), cidr1_mask - 1)];
    }
    return vec![cidr1, cidr2];
}

// Check wether two CIDR blocks are adjascent
fn is_adjascent(cidr1: String, cidr2: String) -> bool {
    let cidr2_net = ipv4todec(calc_network_addr(cidr2.clone()));
    let cidr1_brd = ipv4todec(calc_broadcast_addr(cidr1.clone()));
    return cidr1_brd + 1 == cidr2_net;
}

fn main() {
    let stdin = io::stdin();
    let mut cidrs = Vec::new();
    for line in stdin.lock().lines() {
        //println!("{}", dectoipv4(ipv4todec(line.unwrap())+1));
        cidrs.push(line.unwrap());
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
    }
    /*let args: Vec<String> = env::args().collect();
    let dec = dectoipv4(ipv4todec(&args[1]));
    println!("{}", dec);*/
}
