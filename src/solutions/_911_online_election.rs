use std::collections::HashMap;

struct TopVotedCandidate {
    times: Vec<i32>,
    leaders: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let n = persons.len();
        let mut hm: HashMap<i32, usize> = HashMap::new();
        let mut leader: (usize, i32) = (0, 0);
        let mut leaders = vec![];
        for i in 0..n {
            let count = hm.entry(persons[i]).or_default();
            *count += 1;
            if *count >= leader.0 {
                leader = (*count, persons[i]);
            }
            leaders.push(leader.1);
        }
        TopVotedCandidate { times, leaders }
    }

    fn q(&self, t: i32) -> i32 {
        let i = match self.times.binary_search(&t) {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        self.leaders[i]
    }
}

#[test]
fn test() {
    let persons = vec![0, 1, 1, 0, 0, 1, 0];
    let times = vec![0, 5, 10, 15, 20, 25, 30];
    let tvc = TopVotedCandidate::new(persons, times);
    assert_eq!(tvc.q(3), 0);
    assert_eq!(tvc.q(12), 1);
    assert_eq!(tvc.q(25), 1);
    assert_eq!(tvc.q(15), 0);
    assert_eq!(tvc.q(24), 0);
    assert_eq!(tvc.q(8), 1);
}
