struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        if 0 < n1 && 0 < n2 && visited.insert((0, 0)) {
            queue.push((Reverse(nums1[0] + nums2[0]), 0, 0));
        } else {
            return vec![];
        }
        let mut res = vec![];
        while k > 0 {
            if let Some((_, i, j)) = queue.pop() {
                res.push(vec![nums1[i], nums2[j]]);
                if i + 1 < n1 && visited.insert((i + 1, j)) {
                    queue.push((Reverse(nums1[i + 1] + nums2[j]), i + 1, j));
                }
                if j + 1 < n2 && visited.insert((i, j + 1)) {
                    queue.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
                }
                k -= 1;
            } else {
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums1 = vec![1, 7, 11];
    let nums2 = vec![2, 4, 6];
    let k = 3;
    let mut res = vec_vec_i32![[1, 2], [1, 4], [1, 6]];
    let mut ans = Solution::k_smallest_pairs(nums1, nums2, k);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let nums1 = vec![1, 1, 2];
    let nums2 = vec![1, 2, 3];
    let k = 2;
    let mut res = vec_vec_i32![[1, 1], [1, 1]];
    let mut ans = Solution::k_smallest_pairs(nums1, nums2, k);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
    let nums1 = vec![1, 2];
    let nums2 = vec![3];
    let k = 3;
    let mut res = vec_vec_i32![[1, 3], [2, 3]];
    let mut ans = Solution::k_smallest_pairs(nums1, nums2, k);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
