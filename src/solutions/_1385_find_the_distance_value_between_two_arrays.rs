struct Solution;

impl Solution {
    fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut count: Vec<Vec<i32>> = vec![vec![0; 2001]; 2];
        for x in arr1 {
            count[0][(x + 1000) as usize] += 1;
        }
        for x in arr2 {
            count[1][(x + 1000) as usize] += 1;
        }
        let mut res = 0;
        for i in 0i32..=2000i32 {
            let count_i = count[0][i as usize];
            if count_i != 0 {
                let mut valid = true;
                for j in 0.max(i - d) as usize..=2000.min(i + d) as usize {
                    if count[1][j] != 0 {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    res += count_i;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let arr1 = vec![4, 5, 8];
    let arr2 = vec![10, 9, 1, 8];
    let d = 2;
    let res = 2;
    assert_eq!(Solution::find_the_distance_value(arr1, arr2, d), res);
    let arr1 = vec![1, 4, 2, 3];
    let arr2 = vec![-4, -3, 6, 10, 20, 30];
    let d = 3;
    let res = 2;
    assert_eq!(Solution::find_the_distance_value(arr1, arr2, d), res);
    let arr1 = vec![2, 1, 100, 3];
    let arr2 = vec![-5, -2, 10, -3, 7];
    let d = 6;
    let res = 1;
    assert_eq!(Solution::find_the_distance_value(arr1, arr2, d), res);
}
