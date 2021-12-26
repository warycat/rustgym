use askama::Template;
use rustgym_schema::*;

#[derive(Template, new)]
#[template(path = "sitemap.j2")]
pub struct SitemapContext {
    pub leetcode_rows: Vec<LeetcodeDescription>,
    pub adventofcode_rows: Vec<AdventOfCodeDescription>,
    pub google_rows: Vec<GoogleProblem>,
}

#[derive(Template, new)]
#[template(path = "robots.j2")]
pub struct RobotsContext {}
