use rustgym_schema::LeetcodeQuestion;
use std::collections::HashMap;

type Tags = HashMap<i32, Vec<Tag>>;
type Tag = (String, String);

pub struct LeetcodeData {
    list_url: &'static str,
    tag_url: &'static str,
}

impl LeetcodeData {
    pub fn new(list_url: &'static str, tag_url: &'static str) -> Self {
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

    pub fn get_questions(&self) -> Result<Vec<LeetcodeQuestion>, Box<dyn std::error::Error>> {
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
            let question =
                LeetcodeQuestion::new(id as i32, title, slug, level as i32, frontend_id as i32);
            questions.push(question);
        }
        Ok(questions)
    }

    pub fn get_tags(&self) -> Result<Tags, Box<dyn std::error::Error>> {
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
                hm.entry(id as i32)
                    .or_default()
                    .push((slug.to_string(), name.to_string()));
            }
        }
        Ok(hm)
    }
}
