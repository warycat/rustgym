struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type G = HashMap<String, BinaryHeap<Reverse<String>>>;

impl Solution {
    fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut g: G = HashMap::new();
        for ticket in tickets {
            g.entry(ticket[0].clone())
                .or_default()
                .push(Reverse(ticket[1].clone()));
        }
        let mut stack: Vec<String> = vec!["JFK".to_string()];
        while !stack.is_empty() {
            while g.contains_key(stack.last().unwrap())
                && !g.get(stack.last().unwrap()).unwrap().is_empty()
            {
                let airports = g.get_mut(stack.last().unwrap()).unwrap();
                let airport = airports.pop().unwrap().0;
                stack.push(airport);
            }
            res.insert(0, stack.pop().unwrap());
        }
        res
    }
}

#[test]
fn test() {
    let tickets: Vec<Vec<String>> = vec_vec_string![
        ["MUC", "LHR"],
        ["JFK", "MUC"],
        ["SFO", "SJC"],
        ["LHR", "SFO"]
    ];
    let res: Vec<String> = vec_string!["JFK", "MUC", "LHR", "SFO", "SJC"];
    assert_eq!(Solution::find_itinerary(tickets), res);
}
