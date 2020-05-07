struct Solution;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn find_smallest_region(regions: Vec<Vec<String>>, region1: String, region2: String) -> String {
        let mut parent: HashMap<&String, &String> = HashMap::new();
        for list in &regions {
            let n = list.len();
            for i in 1..n {
                parent.insert(&list[i], &list[0]);
            }
        }
        let mut path: HashSet<&String> = HashSet::new();
        let mut p1 = &region1;
        let mut p2 = &region2;
        path.insert(p1);
        while let Some(next) = parent.get(p1) {
            p1 = next;
            path.insert(p1);
        }
        if path.contains(p2) {
            return p2.to_string();
        }
        while let Some(next) = parent.get(p2) {
            p2 = next;
            if path.contains(p2) {
                return p2.to_string();
            }
        }
        "".to_string()
    }
}

#[test]
fn test() {
    let regions = vec_vec_string![
        ["Earth", "North America", "South America"],
        ["North America", "United States", "Canada"],
        ["United States", "New York", "Boston"],
        ["Canada", "Ontario", "Quebec"],
        ["South America", "Brazil"]
    ];
    let region1 = "Quebec".to_string();
    let region2 = "New York".to_string();
    let res = "North America".to_string();
    assert_eq!(
        Solution::find_smallest_region(regions, region1, region2),
        res
    );
    let regions = vec_vec_string![
        ["Earth", "North America", "South America"],
        ["North America", "United States", "Canada"],
        ["United States", "New York", "Boston"],
        ["Canada", "Ontario", "Quebec"],
        ["South America", "Brazil"]
    ];
    let region1 = "Canada".to_string();
    let region2 = "South America".to_string();
    let res = "Earth".to_string();
    assert_eq!(
        Solution::find_smallest_region(regions, region1, region2),
        res
    );
}
