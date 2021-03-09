struct Solution;

impl Solution {
    fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut nums: Vec<i32> = vec![];
        for i in (2..=n).rev() {
            nums.push(i);
        }
        nums.push(1);
        let n = n as usize;
        let m = 2 * n - 1;
        let mut cur = vec![0; m];
        let mut visited: u32 = (1 << n) - 1;
        Self::dfs(0, &mut cur, &mut visited, &nums, m, n);
        cur
    }

    fn dfs(
        start: usize,
        cur: &mut Vec<i32>,
        visited: &mut u32,
        nums: &[i32],
        m: usize,
        n: usize,
    ) -> bool {
        if start == m {
            true
        } else {
            if cur[start] == 0 {
                for i in 0..n {
                    if 1 << i & *visited != 0 {
                        let distance = nums[i] as usize;
                        if distance == 1 {
                            *visited &= !(1 << i);
                            cur[start] = nums[i];
                            let found = Self::dfs(start + 1, cur, visited, nums, m, n);
                            if found {
                                return true;
                            }
                            cur[start] = 0;
                            *visited |= 1 << i;
                        } else {
                            if start + distance < m && cur[start + distance] == 0 {
                                cur[start] = nums[i];
                                cur[start + distance] = nums[i];
                                *visited &= !(1 << i);
                                let found = Self::dfs(start + 1, cur, visited, nums, m, n);
                                if found {
                                    return true;
                                }
                                cur[start + distance] = 0;
                                cur[start] = 0;
                                *visited |= 1 << i;
                            }
                        }
                    }
                }
                false
            } else {
                Self::dfs(start + 1, cur, visited, nums, m, n)
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let res = vec![3, 1, 2, 3, 2];
    assert_eq!(Solution::construct_distanced_sequence(n), res);
    let n = 5;
    let res = vec![5, 3, 1, 4, 3, 5, 2, 4, 2];
    assert_eq!(Solution::construct_distanced_sequence(n), res);
}
