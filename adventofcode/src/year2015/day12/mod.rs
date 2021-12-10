use rustgym_util::*;
use serde_json::json;
use serde_json::Value;
use std::fmt::Write;
use std::io::*;

pub fn solve(reader: &mut dyn BufRead, writer: &mut dyn Write) {
    let mut it = reader.lines().map(|l| l.unwrap());
    let line = it.next().unwrap();
    let val = serde_json::from_str(&line).unwrap();
    let res1 = dfs1(&val);
    let red: Value = json!("red");
    let res2 = dfs2(&val, &red);
    writeln!(writer, "{}", res1).unwrap();
    writeln!(writer, "{}", res2).unwrap();
}

fn dfs1(val: &Value) -> i64 {
    match val {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Object(obj) => obj.iter().map(|(_, v)| dfs1(v)).sum(),
        Value::Array(arr) => arr.iter().map(dfs1).sum(),
        _ => 0,
    }
}

fn dfs2(val: &Value, red: &Value) -> i64 {
    match val {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Object(obj) => {
            if obj.iter().any(|(_, v)| v == red) {
                0
            } else {
                obj.iter().map(|(_, v)| dfs2(v, red)).sum()
            }
        }
        Value::Array(arr) => arr.iter().map(|v| dfs2(v, red)).sum(),
        _ => 0,
    }
}

adventofcode!(test, "input.txt", "output.txt");
