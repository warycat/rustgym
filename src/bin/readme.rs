use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use util::*;

const TITLE: &str = "# Leetcode Solutions in Rust";
const BODY: &str = "
This project demostrates how to create **Data Structures** and to implement **Algorithms** using programming language **Rust**
All the solutions here are crafted with love and their performance beats 99% of other solutions on the leetcode website. Tutorial videos will be added later.

### Please subscribe to our [Rust Gym Youtube Channel](https://www.youtube.com/channel/UCV9HzRLPKjI8SttaIYOygsw) for future videos.

<details><summary>Data Structures</summary>

- Stack & Queue ( Vec, VecDeque )
- Linked List ( Option<Box<ListNode>> )
- Hash Tables ( HashMap, HashSet )
- Tree Tables ( BTreeMap, BTreeSet )
- Binary Search Tree ( Option<Rc<RefCell<TreeNode>>> )
- Binary Heaps & Priority Queue ( BinaryHeap )
- Graphs ( Vec<Vec<usize>> )
- Union Find ( UnionFind )
- Trie ( Trie )
</details>

<details><summary>Algorithms</summary>

- Bit Manipulation & Numbers
- Stability in Sorting
- Heapsort
- Binary Search
- Kth Smallest Elements
- Permutations
- Subsets
- BFS Graph
- DFS Graph
- Dijkstra’s Algorithm
- Tree Traversals
    - BFS
    - DFS
        - in-order
        - pre-order
        - post-order
- Topological Sort
- Detect cycle in an undirected graph
- Detect a cycle in a directed graph
- Count connected components in a graph
- Find strongly connected components in a graph
</details>
";

const CODING_INTERVIEW: &str = "
### Coding Interview
Leetcode is a website where people–mostly software engineers–practice their coding skills. There are 1200+ questions (and growing), each with multiple solutions. Questions are ranked by level of difficulty: easy, medium, and hard. Within the last decade or so, the technical interview process has become formulaic and what some describe “unnatural” for engineers. What people are asked to perform in an interview–solving word or code based teasers, coding on a whiteboard, and being asked to produce clean optimized solutions in a short time frame–is not what they would experience in a daily work environment.
";

const CI: &str = "### leetcode_rs [![Build Status](https://travis-ci.org/warycat/leetcode_rs.svg?branch=master)](https://travis-ci.org/warycat/leetcode_rs)";
const LEETCODE_JSON_URL: &str = "https://leetcode.com/api/problems/algorithms/";
const LEETCODE_TAG_URL: &str = "https://leetcode.com/problems/api/tags/";
const LEETCODE_QUESTION_URL: &str = "https://leetcode.com/problems/";
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
const README_MD: &str = "README.md";
const SRC: &str = "src/solutions";

type Tags = HashMap<i64, Vec<Tag>>;
type Tag = (String, String);

struct RustSolution {
    id: i64,
    filename: String,
}

impl RustSolution {
    fn new(id: i64, filename: String) -> Self {
        RustSolution { id, filename }
    }
}

impl fmt::Display for RustSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[solution]({}/{})", SRC, self.filename)
    }
}

struct RustSolutionList {
    solutions: Vec<RustSolution>,
}

impl RustSolutionList {
    fn new(src_dir: std::path::PathBuf) -> Self {
        let mut solutions: Vec<RustSolution> = vec![];
        for entry in fs::read_dir(src_dir).unwrap() {
            let filename = entry.unwrap().file_name().to_str().unwrap().to_string();
            if let Some(0) = filename.find('_') {
                let s: Vec<String> = filename.split('_').map(|s| s.to_string()).collect();
                let id = s[1].clone().parse::<i64>().unwrap();
                let problem = RustSolution::new(id, filename);
                solutions.push(problem);
            }
        }
        solutions.sort_by_key(|x| x.id);
        RustSolutionList { solutions }
    }
}

struct LeetcodeData {
    list_url: &'static str,
    tag_url: &'static str,
}

impl LeetcodeData {
    fn new(list_url: &'static str, tag_url: &'static str) -> Self {
        LeetcodeData { list_url, tag_url }
    }

    fn get_list_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(self.list_url)?.text()?;
        Ok(resp)
    }

    fn get_tag_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(self.tag_url)?.text()?;
        Ok(resp)
    }

    fn get_questions(&self) -> Result<Vec<LeetcodeQuestion>, Box<dyn std::error::Error>> {
        let json_string = self.get_list_text()?;
        let value: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let pairs = value["stat_status_pairs"].as_array().unwrap();
        let mut questions = vec![];
        for pair in pairs {
            let stat = pair["stat"].as_object().unwrap();
            let frontend_id = stat["frontend_question_id"].as_i64().unwrap();
            let id = stat["question_id"].as_i64().unwrap();
            let title = stat["question__title"].as_str().unwrap();
            let slug = stat["question__title_slug"].as_str().unwrap();
            let difficulty = pair["difficulty"].as_object().unwrap();
            let level = difficulty["level"].as_i64().unwrap();
            questions.push(LeetcodeQuestion::new(id, title, slug, level, frontend_id))
        }
        Ok(questions)
    }

    fn get_tags(&self) -> Result<Tags, Box<dyn std::error::Error>> {
        let json_string = self.get_tag_text()?;
        let value: serde_json::Value = serde_json::from_str(&json_string).unwrap();
        let topics = value["topics"].as_array().unwrap();
        let mut hm: Tags = HashMap::new();
        for topic in topics {
            let slug = topic["slug"].as_str().unwrap();
            let name = topic["name"].as_str().unwrap();
            let questions = topic["questions"].as_array().unwrap();
            for question in questions {
                let id = question.as_i64().unwrap();
                hm.entry(id)
                    .or_default()
                    .push((slug.to_string(), name.to_string()));
            }
        }
        Ok(hm)
    }
}

struct LeetcodeQuestion {
    id: i64,
    title: String,
    slug: String,
    level: i64,
    frontend_id: i64,
}

impl LeetcodeQuestion {
    fn new<S: Into<String>>(id: i64, title: S, slug: S, level: i64, frontend_id: i64) -> Self {
        LeetcodeQuestion {
            id,
            title: title.into(),
            slug: slug.into(),
            level,
            frontend_id,
        }
    }
}

impl fmt::Display for LeetcodeQuestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]({}{})",
            self.title, LEETCODE_QUESTION_URL, self.slug
        )
    }
}

struct LeetcodeQuestionList {
    questions: Vec<LeetcodeQuestion>,
}

impl LeetcodeQuestionList {
    fn new(questions: Vec<LeetcodeQuestion>) -> Self {
        LeetcodeQuestionList { questions }
    }
}

struct Readme {
    headers: Vec<String>,
    solution_list: RustSolutionList,
    question_list: LeetcodeQuestionList,
    tags: Tags,
    footers: Vec<String>,
}

impl Readme {
    fn new(
        headers: Vec<String>,
        solution_list: RustSolutionList,
        question_list: LeetcodeQuestionList,
        tags: Tags,
        footers: Vec<String>,
    ) -> Self {
        Readme {
            headers,
            solution_list,
            question_list,
            tags,
            footers,
        }
    }

    fn table(&self) -> String {
        let solutions = &self.solution_list.solutions;
        let questions = &self.question_list.questions;
        let mut btm: BTreeMap<i64, (String, i64, i64)> = BTreeMap::new();
        let mut hm: HashMap<i64, String> = HashMap::new();
        for question in questions {
            let id = question.id;
            let frontend_id = question.frontend_id;
            btm.insert(id, (question.to_string(), question.level, frontend_id));
        }
        for solution in solutions {
            let id = solution.id;
            hm.insert(id, solution.to_string());
        }
        let mut s = "# All Solutions\n".to_string();
        for level in 1..=3 {
            let mut rows: Vec<(i64, String, String, String)> = vec![];
            let mut n_questions = 0;
            let mut n_solutions = 0;
            for (&id, question) in &btm {
                if question.1 == level {
                    n_questions += 1;
                    let frontend_id = question.2;
                    let no_tags = vec![];
                    let tags = self.tags.get(&id).unwrap_or(&no_tags);
                    let slugs: Vec<String> = tags.iter().map(|tag| tag.0.to_string()).collect();
                    let tag_string = slugs.join(" ");
                    let no_solution = "   ".to_string();
                    let solution = hm.get(&frontend_id).unwrap_or(&no_solution);
                    n_solutions += if hm.get(&frontend_id).is_some() { 1 } else { 0 };
                    rows.push((
                        frontend_id,
                        question.0.to_string(),
                        tag_string,
                        solution.to_string(),
                    ));
                }
            }
            let level_string = match level {
                1 => "Easy",
                2 => "Medium",
                3 => "Hard",
                _ => "",
            };
            let percentage = n_solutions as f64 / n_questions as f64 * 100.0;
            s += &format!(
                "<details><summary>{} {}/{} {:.2}%</summary>\n\n",
                level_string, n_solutions, n_questions, percentage,
            );
            s += &format!(
                "\n|id|{} {} Questions|Tags|{} Solutions|\n",
                n_questions, level_string, n_solutions
            );
            s += "|---|---|---|---|\n";
            rows.sort_by_key(|row| (row.3.to_string(), row.2.to_string(), row.0));
            for row in rows {
                s += &format!("|{}|{}|{}|{}|\n", row.0, row.1, row.2, row.3);
            }
            s += "</details>\n";
        }
        s
    }
}

impl fmt::Display for Readme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        for header in &self.headers {
            s += &format!("{}\n\n", header);
        }
        s += &self.table();
        for footer in &self.footers {
            s += &format!("{}\n\n", footer);
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let leetcode_json = LeetcodeData::new(LEETCODE_JSON_URL, LEETCODE_TAG_URL);
    let questions = leetcode_json.get_questions().unwrap_or_default();
    let question_list = LeetcodeQuestionList::new(questions);
    let tags = leetcode_json.get_tags().unwrap_or_default();
    let cargo_dir = env::var_os(CARGO_MANIFEST_DIR).unwrap();
    let readme_md = Path::new(&cargo_dir).join(README_MD);
    let src_dir = Path::new(&cargo_dir).join(SRC);
    let solution_list = RustSolutionList::new(src_dir);
    let headers = vec_string![TITLE, BODY, CI];
    let footers = vec_string!(CODING_INTERVIEW);
    let readme = Readme::new(headers, solution_list, question_list, tags, footers);
    fs::write(&readme_md, format!("{}", readme)).unwrap();
}
