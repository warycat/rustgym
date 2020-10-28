mod consts;
mod data;
mod description;
mod question;
mod readme;
mod solution;

use consts::*;
use data::*;
use description::*;
use question::*;
use readme::*;
use solution::*;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;

type Tags = HashMap<u64, Vec<Tag>>;
type Tag = (String, String);

fn main() {
    let leetcode_json = LeetcodeData::new(LEETCODE_JSON_URL, LEETCODE_TAG_URL);
    let questions = leetcode_json.get_questions().unwrap_or_default();
    let question_list = LeetcodeQuestionList::new(questions);
    let tags = leetcode_json.get_tags().unwrap_or_default();
    let cargo_dir = env::var_os(CARGO_MANIFEST_DIR).unwrap();
    let readme_md = Path::new(&cargo_dir).join("..").join(README_MD);
    let src_dir = Path::new(&cargo_dir).join("..").join(LEETCODE_SRC);
    let solution_list = RustSolutionList::new(src_dir);
    let desc_dir = Path::new(&cargo_dir).join("..").join(LEETCODE_DESC);
    let description_list = DescriptionList::new(desc_dir);
    let readme = Readme::new(
        TEMPLATE.to_string(),
        solution_list,
        question_list,
        description_list,
        tags,
    );
    fs::write(&readme_md, format!("{}", readme)).unwrap();
}
