struct BrowserHistory {
    backward: Vec<String>,
    forward: Vec<String>,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            backward: vec![homepage],
            forward: vec![],
        }
    }

    fn visit(&mut self, url: String) {
        self.backward.push(url);
        self.forward = vec![];
    }

    fn back(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if self.backward.len() > 1 {
                self.forward.push(self.backward.pop().unwrap());
            }
        }
        self.backward.last().unwrap().to_string()
    }

    fn forward(&mut self, steps: i32) -> String {
        for _ in 0..steps {
            if !self.forward.is_empty() {
                self.backward.push(self.forward.pop().unwrap());
            }
        }
        self.backward.last().unwrap().to_string()
    }
}

#[test]
fn test() {
    let mut obj = BrowserHistory::new("leetcode.com".to_string());
    obj.visit("google.com".to_string());
    obj.visit("facebook.com".to_string());
    obj.visit("youtube.com".to_string());
    assert_eq!(obj.back(1), "facebook.com".to_string());
    assert_eq!(obj.back(1), "google.com".to_string());
    assert_eq!(obj.forward(1), "facebook.com".to_string());
    obj.visit("linkedin.com".to_string());
    assert_eq!(obj.forward(2), "linkedin.com".to_string());
    assert_eq!(obj.back(2), "google.com".to_string());
    assert_eq!(obj.back(2), "leetcode.com".to_string());
}
