use std::collections::BTreeMap;
use std::collections::HashMap;

const SECONDS: [usize; 3] = [60, 3600, 86400];

#[derive(Default)]
struct TweetCounts {
    data: HashMap<String, BTreeMap<usize, usize>>,
}

impl TweetCounts {
    fn new() -> Self {
        TweetCounts::default()
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        let time = time as usize;
        *self
            .data
            .entry(tweet_name)
            .or_default()
            .entry(time)
            .or_default() += 1;
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let seconds = SECONDS[Self::freq_to_index(&freq)];
        let start_time = start_time as usize;
        let end_time = end_time as usize;
        let n = (end_time - start_time) / seconds + 1;
        let mut res = vec![0; n];

        if let Some(counts) = self.data.get(&tweet_name) {
            for (&t, &c) in counts {
                let i = (t - start_time) / seconds;
                if t >= start_time && t <= end_time {
                    res[i] += c as i32;
                }
            }
        }
        res
    }

    fn freq_to_index(freq: &str) -> usize {
        match freq {
            "minute" => 0,
            "hour" => 1,
            "day" => 2,
            _ => panic!(),
        }
    }
}

#[test]
fn test() {
    let mut obj = TweetCounts::new();
    obj.record_tweet("tweet3".to_string(), 0);
    obj.record_tweet("tweet3".to_string(), 60);
    obj.record_tweet("tweet3".to_string(), 10);
    assert_eq!(
        obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 59),
        vec![2]
    );
    assert_eq!(
        obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 60),
        vec![2, 1]
    );
    obj.record_tweet("tweet3".to_string(), 120);
    assert_eq!(
        obj.get_tweet_counts_per_frequency("hour".to_string(), "tweet3".to_string(), 0, 210),
        vec![4]
    );
}
