use std::collections::HashMap;

struct SparseVector {
    data: HashMap<usize, i32>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut data = HashMap::new();
        let n = nums.len();
        for i in 0..n {
            data.insert(i, nums[i]);
        }
        SparseVector { data }
    }

    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut res = 0;
        for (k, &v1) in &self.data {
            if let Some(&v2) = vec.data.get(k) {
                res += v1 * v2;
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums1 = SparseVector::new(vec![1, 0, 0, 2, 3]);
    let nums2 = SparseVector::new(vec![0, 3, 0, 4, 0]);
    let res = 8;
    assert_eq!(nums1.dot_product(nums2), res);
    let nums1 = SparseVector::new(vec![0, 1, 0, 0, 0]);
    let nums2 = SparseVector::new(vec![0, 0, 0, 0, 2]);
    let res = 0;
    assert_eq!(nums1.dot_product(nums2), res);
    let nums1 = SparseVector::new(vec![0, 1, 0, 0, 2, 0, 0]);
    let nums2 = SparseVector::new(vec![1, 0, 0, 0, 3, 0, 4]);
    let res = 6;
    assert_eq!(nums1.dot_product(nums2), res);
}
