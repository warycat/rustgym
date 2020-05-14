struct Solution;
use std::collections::VecDeque;
use std::fmt;

enum Match {
    Team(i32),
    Pair(Box<Match>, Box<Match>),
}

impl Match {
    fn new(n: i32) -> Self {
        let mut matches: VecDeque<Match> = VecDeque::new();
        for i in 1..=n {
            matches.push_back(Match::team(i));
        }
        while matches.len() > 1 {
            let mut temp: VecDeque<Match> = VecDeque::new();
            while !matches.is_empty() {
                let strong = matches.pop_front().unwrap();
                let week = matches.pop_back().unwrap();
                temp.push_back(Self::pair(strong, week));
            }
            matches = temp;
        }
        matches.pop_back().unwrap()
    }
    fn team(rank: i32) -> Self {
        Match::Team(rank)
    }

    fn pair(strong: Match, week: Match) -> Self {
        Match::Pair(Box::new(strong), Box::new(week))
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Match::Team(rank) => write!(f, "{}", rank),
            Match::Pair(a, b) => write!(f, "({},{})", a, b),
        }
    }
}

impl Solution {
    fn find_contest_match(n: i32) -> String {
        Match::new(n).to_string()
    }
}

#[test]
fn test() {
    let n = 2;
    let res = "(1,2)".to_string();
    assert_eq!(Solution::find_contest_match(n), res);
    let n = 4;
    let res = "((1,4),(2,3))".to_string();
    assert_eq!(Solution::find_contest_match(n), res);
    let n = 8;
    let res = "(((1,8),(4,5)),((2,7),(3,6)))".to_string();
    assert_eq!(Solution::find_contest_match(n), res);
}
