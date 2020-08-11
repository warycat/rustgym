use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

type Tweet = (Reverse<usize>, i32);

#[derive(Default)]
struct Twitter {
    time: usize,
    users: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<Tweet>>,
    limit: usize,
}

impl Twitter {
    fn new() -> Self {
        let time = 0;
        let users = HashMap::new();
        let tweets = HashMap::new();
        let limit = 10;
        Twitter {
            time,
            users,
            tweets,
            limit,
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.time += 1;
        let tweet: Tweet = (Reverse(self.time), tweet_id);
        self.tweets.entry(user_id).or_default().push(tweet);
    }

    fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut pq: BinaryHeap<Tweet> = BinaryHeap::with_capacity(self.limit + 1);
        let mut res: Vec<i32> = vec![];
        let followers = self.users.entry(user_id).or_default();
        followers.insert(user_id);
        for &user in followers.iter() {
            let tweets = self.tweets.entry(user).or_default();
            for tweet in tweets {
                pq.push(*tweet);
                if pq.len() > self.limit {
                    pq.pop();
                }
            }
        }
        while !pq.is_empty() {
            let earliest = pq.pop().unwrap();
            res.push(earliest.1);
        }
        res.reverse();
        res
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_default()
            .remove(&followee_id);
    }
}

#[test]
fn test() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
    twitter.unfollow(1, 2);
    assert_eq!(twitter.get_news_feed(1), vec![5]);
}
