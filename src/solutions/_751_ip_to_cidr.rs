struct Solution;

use std::net::Ipv4Addr;

impl Solution {
    fn ip_to_cidr(ip: String, n: i32) -> Vec<String> {
        let mut range = n as i64;
        let mut res: Vec<String> = vec![];
        let addr: Ipv4Addr = ip.parse().unwrap();
        let mut ip: i64 = 0;
        for &byte in addr.octets().iter() {
            ip <<= 8;
            ip |= byte as i64;
        }
        while range > 0 {
            let mut step = ip & -ip;
            while step > range as i64 {
                step >>= 1;
            }
            res.push(Self::cidr(ip, step));
            ip += step;
            range -= step;
        }
        res
    }

    fn cidr(ip: i64, mut step: i64) -> String {
        let addr = Ipv4Addr::new(
            (ip >> 24) as u8,
            (ip >> 16) as u8,
            (ip >> 8) as u8,
            ip as u8,
        );
        let mut len = 0;
        while step > 0 {
            len += 1;
            step >>= 1;
        }
        format!("{}/{}", addr, 33 - len)
    }
}

#[test]
fn test() {
    let ip = "255.0.0.7".to_string();
    let n = 10;
    let cidrs: Vec<String> = vec_string!["255.0.0.7/32", "255.0.0.8/29", "255.0.0.16/32"];
    assert_eq!(Solution::ip_to_cidr(ip, n), cidrs);
}
