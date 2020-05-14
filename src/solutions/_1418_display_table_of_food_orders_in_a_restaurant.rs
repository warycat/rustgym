struct Solution;
use std::collections::BTreeSet;
use std::collections::HashMap;

impl Solution {
    fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut tables: BTreeSet<i32> = BTreeSet::new();
        let mut foods: BTreeSet<&str> = BTreeSet::new();
        let mut counts: HashMap<i32, HashMap<&str, usize>> = HashMap::new();
        for order in &orders {
            let table = order[1].parse::<i32>().unwrap();
            let food = &order[2];
            tables.insert(table);
            foods.insert(food);
            *counts.entry(table).or_default().entry(food).or_default() += 1;
        }
        let mut res = vec![vec!["Table".to_string()]];
        for food in foods.iter() {
            res[0].push((*food).to_string());
        }
        for table in tables {
            let mut row = vec![table.to_string()];
            for food in &foods {
                if let Some(counts) = counts.get(&table) {
                    if let Some(count) = counts.get(food) {
                        row.push(count.to_string());
                    } else {
                        row.push("0".to_string());
                    }
                } else {
                    row.push("0".to_string());
                }
            }
            res.push(row);
        }
        res
    }
}

#[test]
fn test() {
    let orders = vec_vec_string![
        ["David", "3", "Ceviche"],
        ["Corina", "10", "Beef Burrito"],
        ["David", "3", "Fried Chicken"],
        ["Carla", "5", "Water"],
        ["Carla", "5", "Ceviche"],
        ["Rous", "3", "Ceviche"]
    ];
    let res = vec_vec_string![
        ["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
        ["3", "0", "2", "1", "0"],
        ["5", "0", "1", "0", "1"],
        ["10", "1", "0", "0", "0"]
    ];
    assert_eq!(Solution::display_table(orders), res);
    let orders = vec_vec_string![
        ["James", "12", "Fried Chicken"],
        ["Ratesh", "12", "Fried Chicken"],
        ["Amadeus", "12", "Fried Chicken"],
        ["Adam", "1", "Canadian Waffles"],
        ["Brianna", "1", "Canadian Waffles"]
    ];
    let res = vec_vec_string![
        ["Table", "Canadian Waffles", "Fried Chicken"],
        ["1", "2", "0"],
        ["12", "0", "3"]
    ];
    assert_eq!(Solution::display_table(orders), res);
    let orders = vec_vec_string![
        ["Laura", "2", "Bean Burrito"],
        ["Jhon", "2", "Beef Burrito"],
        ["Melissa", "2", "Soda"]
    ];
    let res = vec_vec_string![
        ["Table", "Bean Burrito", "Beef Burrito", "Soda"],
        ["2", "1", "1", "1"]
    ];
    assert_eq!(Solution::display_table(orders), res);
}
