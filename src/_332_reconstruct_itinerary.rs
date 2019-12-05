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
        let mut stack: Vec<String> = vec![];
        stack.push("JFK".to_string());
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
    let tickets: Vec<Vec<String>> = [
        ["MUC", "LHR"],
        ["JFK", "MUC"],
        ["SFO", "SJC"],
        ["LHR", "SFO"],
    ]
    .iter()
    .map(|v| v.iter().map(|s| s.to_string()).collect())
    .collect();
    let res: Vec<String> = ["JFK", "MUC", "LHR", "SFO", "SJC"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::find_itinerary(tickets), res);
}
