use super::schema::google_problem;
use regex::Regex;
use std::path::Path;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "google_problem"]
pub struct GoogleProblem {
    pub id: i32,
    pub division: i32,
    pub year: i32,
    pub round: i32,
    pub number: i32,
    pub title: String,
    pub problem: String,
    pub solution: String,
    pub analysis: String,
}

impl GoogleProblem {
    pub fn new(division: i32, year: i32, round: i32, number: i32, title: String) -> Self {
        let id = ((division * 10000 + year) * 100 + round) * 100 + number;
        let problem = "".to_string();
        let solution = "".to_string();
        let analysis = "".to_string();
        GoogleProblem {
            id,
            division,
            year,
            round,
            number,
            title,
            problem,
            solution,
            analysis,
        }
    }

    pub fn with_path(path: &Path) -> GoogleProblem {
        let re = Regex::new(r".*(codejam|kickstart).*year(\d+).*round(\d+).*_(\d+)_([a-z_]+).*")
            .unwrap();
        let caps = re.captures(path.to_str().unwrap()).unwrap();
        let division = match &caps[1] {
            "codejam" => 1,
            "kickstart" => 2,
            _ => 0,
        };
        let year = caps[2].parse::<i32>().unwrap();
        let round = caps[3].parse::<i32>().unwrap();
        let number = caps[4].parse::<i32>().unwrap();
        let title = caps
            .get(5)
            .unwrap()
            .as_str()
            .chars()
            .map(|c| if c == '_' { ' ' } else { c })
            .collect();
        GoogleProblem::new(division, year, round, number, title)
    }
}
