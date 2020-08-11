struct NumArray {
    prefix_sums: Vec<i32>,
}

impl NumArray {
    fn new(mut nums: Vec<i32>) -> Self {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        NumArray { prefix_sums: nums }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let j: usize = j as usize;
        let i: usize = i as usize;
        if i == 0 {
            self.prefix_sums[j]
        } else {
            self.prefix_sums[j] - self.prefix_sums[i - 1]
        }
    }
}

#[test]
fn test() {
    let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(obj.sum_range(0, 2), 1);
    assert_eq!(obj.sum_range(2, 5), -1);
    assert_eq!(obj.sum_range(0, 5), -3);
}
