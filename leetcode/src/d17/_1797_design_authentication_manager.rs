use std::collections::HashMap;

struct AuthenticationManager {
    tokens: HashMap<String, i32>,
    ttl: i32,
}

impl AuthenticationManager {
    fn new(ttl: i32) -> Self {
        let tokens = HashMap::new();
        AuthenticationManager { tokens, ttl }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        self.tokens.insert(token_id, current_time);
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(time) = self.tokens.get_mut(&token_id) {
            if *time + self.ttl > current_time {
                *time = current_time;
            }
        }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        let mut res = 0;
        for time in self.tokens.values() {
            if time + self.ttl > current_time {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let mut obj = AuthenticationManager::new(5);
    obj.renew("aaa".to_string(), 1);
    obj.generate("aaa".to_string(), 2);
    assert_eq!(obj.count_unexpired_tokens(6), 1);
    obj.generate("bbb".to_string(), 7);
    obj.renew("aaa".to_string(), 8);
    obj.renew("bbb".to_string(), 10);
    assert_eq!(obj.count_unexpired_tokens(15), 0);
}
