struct Solution;

type Person = (i32, usize);

impl Solution {
    fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut people: Vec<Person> = vec![];
        let mut res: Vec<Vec<i32>> = vec![];
        for (id, &group_size) in group_sizes.iter().enumerate() {
            people.push((group_size, id))
        }
        people.sort_unstable();
        let mut group: Vec<i32> = vec![];
        for p in people {
            group.push(p.1 as i32);
            if group.len() == p.0 as usize {
                res.push(group);
                group = vec![];
            }
        }
        res
    }
}

#[test]
fn test() {
    let group_sizes = vec![3, 3, 3, 3, 3, 1, 3];
    let res = vec_vec_i32![[5], [0, 1, 2], [3, 4, 6]];
    assert_eq!(Solution::group_the_people(group_sizes), res);
    let group_sizes = vec![2, 1, 3, 3, 3, 2];
    let res = vec_vec_i32![[1], [0, 5], [2, 3, 4]];
    assert_eq!(Solution::group_the_people(group_sizes), res);
}
