struct Solution;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

impl Solution {
    fn valid_ip_address(ip: String) -> String {
        if Self::is_ipv4(&ip) {
            return "IPv4".to_string();
        }
        if Self::is_ipv6(&ip) {
            return "IPv6".to_string();
        }
        "Neither".to_string()
    }

    fn is_ipv4(ip: &str) -> bool {
        if let Ok(v4) = ip.parse::<Ipv4Addr>() {
            v4.to_string() == ip
        } else {
            false
        }
    }

    fn is_ipv6(ip: &str) -> bool {
        if ip.split(':').any(|p| p.is_empty()) {
            return false;
        }
        ip.parse::<Ipv6Addr>().is_ok()
    }
}

#[test]
fn test() {
    let ip = "172.16.254.1".to_string();
    let res = "IPv4".to_string();
    assert_eq!(Solution::valid_ip_address(ip), res);
    let ip = "2001:0db8:85a3:0:0:8A2E:0370:7334".to_string();
    let res = "IPv6".to_string();
    assert_eq!(Solution::valid_ip_address(ip), res);
    let ip = "256.256.256.256".to_string();
    let res = "Neither".to_string();
    assert_eq!(Solution::valid_ip_address(ip), res);
    let ip = "01.01.01.01".to_string();
    let res = "Neither".to_string();
    assert_eq!(Solution::valid_ip_address(ip), res);
    let ip = "2001:db8:85a3:0::8a2E:0370:7334".to_string();
    let res = "Neither".to_string();
    assert_eq!(Solution::valid_ip_address(ip), res);
}
