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
    let list1: Vec<String> = vec_string!["Shogun", "Tapioca Express", "Burger King", "KFC"];
    let list2: Vec<String> = vec_string![
        "Piatti",
        "The Grill at Torrey Pines",
        "Hungry Hunter Steakhouse",
        "Shogun"
    ];
    let res: Vec<String> = vec_string!["Shogun"];
    assert_eq!(Solution::find_restaurant(list1, list2), res);
    let list1: Vec<String> = vec_string!["Shogun", "Tapioca Express", "Burger King", "KFC"];
    let list2: Vec<String> = vec_string!["KFC", "Shogun", "Burger King"];
    let res: Vec<String> = vec_string!["Shogun"];
    assert_eq!(Solution::find_restaurant(list1, list2), res);
}
