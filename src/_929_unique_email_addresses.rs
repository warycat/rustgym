struct Solution;

use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Email {
    local_name: String,
    domain_name: String,
}

impl Email {
    fn new(s: String) -> Self {
        let mut iter = s.split('@');
        let left: String = iter.next().unwrap().to_string();
        let domain_name: String = iter.next().unwrap().to_string();
        let mut local_name: String = left.split('+').next().unwrap().to_string();
        local_name.retain(|c| c != '.');
        Email {
            local_name,
            domain_name,
        }
    }
}

impl Solution {
    fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut hs: HashSet<Email> = HashSet::new();
        let emails: Vec<Email> = emails
            .iter()
            .map(|email| Email::new(email.to_string()))
            .collect();
        for email in emails {
            hs.insert(email);
        }
        hs.len() as i32
    }
}

#[test]
fn test() {
    let emails: Vec<String> = vec_string![
        "test.email+alex@leetcode.com",
        "test.e.mail+bob.cathy@leetcode.com",
        "testemail+david@lee.tcode.com"
    ];
    assert_eq!(Solution::num_unique_emails(emails), 2);
}
