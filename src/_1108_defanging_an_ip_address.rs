struct Solution;

impl Solution {
    fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

#[test]
fn test() {
    let address = "1.1.1.1".to_string();
    let res = "1[.]1[.]1[.]1".to_string();
    assert_eq!(Solution::defang_i_paddr(address), res);
    let address = "255.100.50.0".to_string();
    let res = "255[.]100[.]50[.]0".to_string();
    assert_eq!(Solution::defang_i_paddr(address), res);
}
