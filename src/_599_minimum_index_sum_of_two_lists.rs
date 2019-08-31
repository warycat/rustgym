struct Solution;

use std::collections::HashMap;
use std::usize;

impl Solution {
    fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut hm: HashMap<&str, usize> = HashMap::new();
        let mut min = usize::MAX;
        let mut res: Vec<String> = vec![];
        for i in 0..list1.len() {
            hm.insert(&list1[i], i);
        }
        for j in 0..list2.len() {
            if let Some(i) = hm.get(list2[j].as_str()) {
                let sum = i + j;
                if sum < min {
                    min = sum;
                    res.clear();
                }
                if sum == min {
                    res.push(list2[j].to_string());
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let list1: Vec<String> = ["Shogun", "Tapioca Express", "Burger King", "KFC"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let list2: Vec<String> = [
        "Piatti",
        "The Grill at Torrey Pines",
        "Hungry Hunter Steakhouse",
        "Shogun",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let res: Vec<String> = ["Shogun"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::find_restaurant(list1, list2), res);
    let list1: Vec<String> = ["Shogun", "Tapioca Express", "Burger King", "KFC"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let list2: Vec<String> = ["KFC", "Shogun", "Burger King"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let res: Vec<String> = ["Shogun"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::find_restaurant(list1, list2), res);
}
