struct Solution;

impl Solution {
    fn judge_point24(nums: Vec<i32>) -> bool {
        let nums: Vec<f64> = nums.into_iter().map(|x| x as f64).collect();
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        let d = nums[3];
        let ab_cd = Self::values(&Self::values(&[a], &[b]), &Self::values(&[c], &[d]));
        let ac_bd = Self::values(&Self::values(&[a], &[c]), &Self::values(&[b], &[d]));
        let ad_bc = Self::values(&Self::values(&[b], &[d]), &Self::values(&[b], &[c]));
        let a_bcd = Self::values(&[a], &Self::rotate(&[b], &[c], &[d]));
        let b_acd = Self::values(&[b], &Self::rotate(&[a], &[c], &[d]));
        let c_abd = Self::values(&[c], &Self::rotate(&[a], &[b], &[d]));
        let d_abc = Self::values(&[d], &Self::rotate(&[a], &[b], &[c]));

        vec![ab_cd, ac_bd, ad_bc, a_bcd, b_acd, c_abd, d_abc]
            .into_iter()
            .flatten()
            .any(|x| (x - 24.0).abs() < 0.000001)
    }

    fn rotate(a: &[f64], b: &[f64], c: &[f64]) -> Vec<f64> {
        let ab_c = Self::values(&Self::values(a, b), c);
        let ac_b = Self::values(&Self::values(a, c), b);
        let bc_a = Self::values(&Self::values(b, c), a);
        vec![ab_c, ac_b, bc_a].into_iter().flatten().collect()
    }

    fn values(a: &[f64], b: &[f64]) -> Vec<f64> {
        let mut res = vec![];
        for i in 0..a.len() {
            for j in 0..b.len() {
                let x = a[i];
                let y = b[j];
                res.push(x + y);
                res.push(x - y);
                res.push(x * y);
                res.push(x / y);
                res.push(y - x);
                res.push(y / x);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![4, 1, 8, 7];
    let res = true;
    assert_eq!(Solution::judge_point24(nums), res);
    let nums = vec![3, 3, 8, 8];
    let res = true;
    assert_eq!(Solution::judge_point24(nums), res);
}
