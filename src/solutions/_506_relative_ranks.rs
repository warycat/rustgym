struct Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Athlete {
    index: usize,
    score: i32,
    rank: String,
}

impl Solution {
    fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let n = nums.len();
        let mut a: Vec<Athlete> = vec![];
        for i in 0..n {
            a.push(Athlete {
                index: i,
                score: nums[i],
                rank: "".to_string(),
            });
        }
        a.sort_unstable_by(|a, b| b.score.cmp(&a.score));
        for i in 0..n {
            a[i].rank = match i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => format!("{}", i + 1),
            }
        }
        a.sort_unstable_by(|a, b| a.index.cmp(&b.index));
        a.into_iter().map(|a| a.rank).collect()
    }
}

#[test]
fn test() {
    let nums = vec![5, 4, 3, 2, 1];
    let res = vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"];
    assert_eq!(Solution::find_relative_ranks(nums), res);
}
