// struct Solution;

// impl Solution {
//     fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
//         let n = n as usize;
//         let m = m as usize;
//         let mut k = m;
//         let mut new_group = vec![];
//         for i in 0..n {
//             if group[i] == -1 {
//                 new_group.push(k);
//                 k += 1;
//             } else {
//                 new_group.push(group[i] as usize);
//             }
//         }

//         let mut before_items: Vec<Vec<usize>> = before_items
//             .into_iter()
//             .map(|v| v.into_iter().map(|i| i as usize).collect())
//             .collect();
//         let mut groups = vec![vec![]];
//         for i in 0..n {
//             let g = new_group[i];
//             groups[g].push(i);
//         }
//         let mut res = vec![];

//         if !Self::topological_sort(k, n, &new_group, &groups, &before_items, &mut res) {
//             return vec![];
//         }

//         res
//     }

//     fn topological_sort(
//         k: usize,
//         n: usize,
//         group: &[usize],
//         groups: &[Vec<usize>],
//         before_items: &[Vec<usize>],
//         stack: &mut Vec,
//     ) -> bool {
//     }
// }
