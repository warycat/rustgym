struct Solution;

fn distance(v: &[i32]) -> i32 {
    v[0] * v[0] + v[1] * v[1]
}

fn quick_select(a: &mut Vec<Vec<i32>>, l: usize, r: usize, k: usize) {
    if l == r {
        return;
    }
    let index = partition(a, l, r);
    let rank = index - l + 1;
    if rank > k {
        quick_select(a, l, index - 1, k);
    } else if rank < k {
        quick_select(a, index + 1, r, k - rank);
    }
}

fn partition(a: &mut Vec<Vec<i32>>, l: usize, r: usize) -> usize {
    let x = distance(&a[r]);
    let mut i = l;
    for j in l..r {
        if distance(&a[j]) <= x {
            a.swap(i, j);
            i += 1;
        }
    }
    a.swap(i, r);
    i
}

impl Solution {
    fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let n = points.len();
        quick_select(&mut points, 0, n - 1, k as usize);
        points.resize(k as usize, vec![]);
        points
    }
}

#[test]
fn test() {
    let points: Vec<Vec<i32>> = [[1, 3], [-2, 2]].iter().map(|v| v.to_vec()).collect();
    let res: Vec<Vec<i32>> = [[-2, 2]].iter().map(|v| v.to_vec()).collect();
    assert_eq!(Solution::k_closest(points, 1), res);
}
