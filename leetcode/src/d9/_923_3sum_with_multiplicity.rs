struct Solution;

const MOD: usize = 1_000_000_007;

impl Solution {
    fn three_sum_multi(a: Vec<i32>, target: i32) -> i32 {
        let mut count = vec![0; 101];
        for x in a {
            count[x as usize] += 1;
        }
        let mut res = 0;
        for x in 0..101 {
            for y in x + 1..101 {
                if x + y >= target as usize {
                    break;
                }
                for z in y + 1..101 {
                    if x + y + z > target as usize {
                        break;
                    }
                    if x + y + z == target as usize {
                        res += count[x] * count[y] * count[z];
                        res %= MOD;
                    }
                }
            }
        }
        for x in 0..101 {
            for y in x + 1..101 {
                if x + x + y != target as usize {
                    continue;
                }
                if count[x] > 1 {
                    res += count[x] * (count[x] - 1) / 2 * count[y];
                    res %= MOD;
                }
            }
        }
        for x in 0..101 {
            for y in x + 1..101 {
                if x + y + y != target as usize {
                    continue;
                }
                if count[y] > 1 {
                    res += count[x] * count[y] * (count[y] - 1) / 2;
                    res %= MOD;
                }
            }
        }
        for x in 0..101 {
            if x + x + x != target as usize {
                continue;
            }
            if count[x] > 2 {
                res += count[x] * (count[x] - 1) * (count[x] - 2) / 6;
                res %= MOD;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let a = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
    let target = 8;
    let res = 20;
    assert_eq!(Solution::three_sum_multi(a, target), res);
    let a = vec![1, 1, 2, 2, 2, 2];
    let target = 5;
    let res = 12;
    assert_eq!(Solution::three_sum_multi(a, target), res);
}
