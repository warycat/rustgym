use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;
use std::vec;
use threadpool::ThreadPool;

struct Solution {}

fn hostname(url: &str) -> String {
    let parts: Vec<&str> = url.split('/').collect();
    parts[2].to_string()
}

impl Solution {
    fn crawl(start_url: String, html_parser: HtmlParser) -> Vec<String> {
        let pool = ThreadPool::new(4);
        let mut queue = vec![];
        let mut visited = HashSet::new();
        let start_hostname = hostname(&start_url);
        if visited.insert(start_url.clone()) {
            queue.push(start_url);
        }
        while !queue.is_empty() {
            let (tx, rx) = channel();
            while let Some(url) = queue.pop() {
                let tx = tx.clone();
                let html_parser = html_parser.clone();
                pool.execute(move || {
                    let urls = html_parser.get_urls(url);
                    tx.send(urls).unwrap();
                });
            }
            pool.join();
            let next_urls: Vec<Vec<String>> = rx.try_iter().collect();
            for url in next_urls.into_iter().flatten() {
                if hostname(&url) == start_hostname && visited.insert(url.clone()) {
                    queue.push(url);
                }
            }
        }
        visited.into_iter().collect()
    }
}

#[derive(Clone)]
struct HtmlParser {
    graph: HashMap<String, Vec<String>>,
}

impl HtmlParser {
    fn new(urls: Vec<String>, edges: Vec<Vec<i32>>) -> Self {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for e in edges {
            let u = urls[e[0] as usize].clone();
            let v = urls[e[1] as usize].clone();
            graph.entry(u).or_default().push(v);
        }
        HtmlParser { graph }
    }

    fn get_urls(&self, url: String) -> Vec<String> {
        sleep(Duration::from_millis(15));
        if let Some(v) = self.graph.get(&url) {
            v.clone()
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    let urls = vec_string![
        "http://news.yahoo.com",
        "http://news.yahoo.com/news",
        "http://news.yahoo.com/news/topics/",
        "http://news.google.com",
        "http://news.yahoo.com/us"
    ];
    let edges = vec_vec_i32![[2, 0], [2, 1], [3, 2], [3, 1], [0, 4]];
    let start_url = "http://news.yahoo.com/news/topics/".to_string();
    let html_parse = HtmlParser::new(urls, edges);
    let mut res = Solution::crawl(start_url, html_parse);
    let mut ans = vec_string![
        "http://news.yahoo.com",
        "http://news.yahoo.com/news",
        "http://news.yahoo.com/news/topics/",
        "http://news.yahoo.com/us"
    ];
    res.sort();
    ans.sort();
    assert_eq!(res, ans);

    let urls = vec_string![
        "http://news.yahoo.com",
        "http://news.yahoo.com/news",
        "http://news.yahoo.com/news/topics/",
        "http://news.google.com"
    ];
    let edges = vec_vec_i32![[0, 2], [2, 1], [3, 2], [3, 1], [3, 0]];
    let start_url = "http://news.google.com".to_string();
    let html_parse = HtmlParser::new(urls, edges);
    let mut res = Solution::crawl(start_url, html_parse);
    let mut ans = vec_string!["http://news.google.com"];
    res.sort();
    ans.sort();
    assert_eq!(res, ans);
}
