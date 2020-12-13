pub mod data;
pub mod question;

pub const LEETCODE_JSON_URL: &str = "https://leetcode.com/api/problems/algorithms/";
pub const LEETCODE_TAG_URL: &str = "https://leetcode.com/problems/api/tags/";
pub const LEETCODE_QUESTION_URL: &str = "https://leetcode.com/problems/";

pub use data::*;
pub use question::*;
