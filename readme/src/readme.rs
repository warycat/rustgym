use super::*;

pub struct Readme {
    template: String,
    solution_list: RustSolutionList,
    question_list: LeetcodeQuestionList,
    description_list: DescriptionList,
    tags: Tags,
}

impl Readme {
    pub fn new(
        template: String,
        solution_list: RustSolutionList,
        question_list: LeetcodeQuestionList,
        description_list: DescriptionList,
        tags: Tags,
    ) -> Self {
        Readme {
            template,
            solution_list,
            question_list,
            description_list,
            tags,
        }
    }

    fn table(&self) -> String {
        let solutions = &self.solution_list.solutions;
        let questions = &self.question_list.questions;
        let descriptions = &self.description_list.descriptions;
        let mut btm: BTreeMap<u64, (String, u64, u64)> = BTreeMap::new();
        let mut solution_map: HashMap<u64, String> = HashMap::new();
        let mut description_map: HashMap<u64, String> = HashMap::new();
        for question in questions {
            let id = question.id;
            let frontend_id = question.frontend_id;
            btm.insert(id, (question.to_string(), question.level, frontend_id));
        }
        for solution in solutions {
            let id = solution.id;
            solution_map.insert(id, solution.to_string());
        }
        for description in descriptions {
            let id = description.id;
            description_map.insert(id, description.to_string());
        }
        let mut s = "# All Solutions\n".to_string();
        for level in 1..=3 {
            let mut rows: Vec<(u64, String, String, String, String)> = vec![];
            let mut n_questions = 0;
            let mut n_solutions = 0;
            for (&id, question) in &btm {
                if question.1 == level {
                    n_questions += 1;
                    let frontend_id = question.2;
                    let frontend_id_str = frontend_id.to_string();
                    let no_tags = vec![];
                    let tags = self.tags.get(&id).unwrap_or(&no_tags);
                    let slugs: Vec<String> = tags.iter().map(|tag| tag.0.to_string()).collect();
                    let tag_string = slugs.join(" ");
                    let empty = "   ".to_string();
                    let solution = solution_map.get(&frontend_id).unwrap_or(&empty);
                    let description = description_map
                        .get(&frontend_id)
                        .unwrap_or(&frontend_id_str);
                    n_solutions += if solution_map.get(&frontend_id).is_some() {
                        1
                    } else {
                        0
                    };
                    rows.push((
                        frontend_id,
                        question.0.to_string(),
                        tag_string,
                        solution.to_string(),
                        description.to_string(),
                    ));
                }
            }
            let level_string = match level {
                1 => "Easy",
                2 => "Medium",
                3 => "Hard",
                _ => "",
            };
            let link_string = match level {
                1 => "leetcode_easy",
                2 => "leetcode_medium",
                3 => "leetcode_hard",
                _ => "",
            };

            let percentage = (1.0 - n_solutions as f64 / n_questions as f64) * 100.0;
            s += &format!(
                "# Leetcode {} {}/{} {:.2} <a name='{}'></a>\n",
                level_string,
                n_questions - n_solutions,
                n_questions,
                percentage,
                link_string,
            );
            s += &format!(
                "\n|id|{} {} Questions|Tags|{} Solutions|\n",
                n_questions, level_string, n_solutions
            );
            s += "|---|---|---|---|\n";
            rows.sort_by_key(|row| (row.3.to_string(), row.2.to_string(), row.0));
            for row in rows {
                s += &format!("|{}|{}|{}|{}|\n", row.4, row.1, row.2, row.3);
            }
            s += "\n";
        }
        s
    }
}

impl fmt::Display for Readme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        s += &self.template;
        s += &self.table();
        write!(f, "{}", s)
    }
}
