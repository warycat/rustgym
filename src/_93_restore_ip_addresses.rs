struct Solution;

impl Solution {
    fn restore_ip_addresses(s: String) -> Vec<String> {
        let n = s.len();
        let mut v: Vec<u8> = vec![0; 4];
        let mut res: Vec<String> = vec![];
        for i in 1..4 {
            if i + 3 > n {
                break;
            }
            if let Ok(a) = s[0..i].parse::<u8>() {
                v[0] = a;
            } else {
                break;
            }
            for j in 1..4 {
                if i + j + 2 > n {
                    break;
                }
                if let Ok(b) = s[i..i + j].parse::<u8>() {
                    v[1] = b;
                } else {
                    break;
                }
                for k in 1..4 {
                    if i + j + k + 1 > n {
                        break;
                    }
                    if let Ok(c) = s[i + j..i + j + k].parse::<u8>() {
                        v[2] = c;
                    } else {
                        break;
                    }
                    for l in 1..4 {
                        if i + j + k + l != n {
                            continue;
                        }
                        if let Ok(d) = s[i + j + k..n].parse::<u8>() {
                            v[3] = d;
                            let ip = format!("{}.{}.{}.{}", v[0], v[1], v[2], v[3]);
                            if ip.len() == n + 3 {
                                res.push(ip);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "25525511135".to_string();
    let res: Vec<String> = vec_string!["255.255.11.135", "255.255.111.35"];
    assert_eq!(Solution::restore_ip_addresses(s), res);
    let s = "010010".to_string();
    let res: Vec<String> = vec_string!["0.10.0.10", "0.100.1.0"];
    assert_eq!(Solution::restore_ip_addresses(s), res);
}
