struct Solution;
use std::usize;

impl Solution {
    fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        products.sort_unstable();
        let n = products.len();
        let mut prefix = "".to_string();
        let mut start = 0;
        for c in search_word.chars() {
            prefix.push(c);
            start = start
                + products[start..]
                    .binary_search(&prefix)
                    .unwrap_or_else(|p| p);
            let mut list: Vec<String> = vec![];
            let end = usize::min(start + 3, n);
            for i in start..end {
                if products[i].starts_with(&prefix) {
                    list.push(products[i].clone());
                }
            }
            res.push(list);
        }
        res
    }
}

#[test]
fn test() {
    let products = vec_string!["mobile", "mouse", "moneypot", "monitor", "mousepad"];
    let search_word = "mouse".to_string();
    let res: Vec<Vec<String>> = vec_vec_string![
        ["mobile", "moneypot", "monitor"],
        ["mobile", "moneypot", "monitor"],
        ["mouse", "mousepad"],
        ["mouse", "mousepad"],
        ["mouse", "mousepad"]
    ];
    assert_eq!(Solution::suggested_products(products, search_word), res);
    let products = vec_string!["havana"];
    let search_word = "havana".to_string();
    let res: Vec<Vec<String>> = vec_vec_string![
        ["havana"],
        ["havana"],
        ["havana"],
        ["havana"],
        ["havana"],
        ["havana"]
    ];
    assert_eq!(Solution::suggested_products(products, search_word), res);
    let products = vec_string!["bags", "baggage", "banner", "box", "cloths"];
    let search_word = "bags".to_string();
    let res: Vec<Vec<String>> = vec_vec_string![
        ["baggage", "bags", "banner"],
        ["baggage", "bags", "banner"],
        ["baggage", "bags"],
        ["bags"]
    ];
    assert_eq!(Solution::suggested_products(products, search_word), res);
    let products = vec_string!["havana"];
    let search_word = "tatiana".to_string();
    let res: Vec<Vec<String>> = vec_vec_string![[], [], [], [], [], [], []];
    assert_eq!(Solution::suggested_products(products, search_word), res);
}
