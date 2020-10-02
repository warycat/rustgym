use std::collections::HashMap;
use std::collections::HashSet;

struct ThroneInheritance {
    root: String,
    nodes: HashMap<String, Vec<String>>,
    dead: HashSet<String>,
}

impl ThroneInheritance {
    fn new(root: String) -> Self {
        let nodes = HashMap::new();
        let dead = HashSet::new();
        ThroneInheritance { root, nodes, dead }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.nodes.entry(parent_name).or_default().push(child_name);
    }

    fn death(&mut self, name: String) {
        self.dead.insert(name);
    }

    fn get_inheritance_order(&self) -> Vec<String> {
        let mut res = vec![];
        self.dfs(&self.root, &mut res);
        res
    }

    fn dfs(&self, name: &str, all: &mut Vec<String>) {
        if !self.dead.contains(name) {
            all.push(name.to_string())
        }
        for child in self.nodes.get(name).unwrap_or(&vec![]) {
            self.dfs(child, all);
        }
    }
}

#[test]
fn test() {
    let mut t = ThroneInheritance::new("king".to_string());
    t.birth("king".to_string(), "andy".to_string());
    t.birth("king".to_string(), "bob".to_string());
    t.birth("king".to_string(), "catherine".to_string());
    t.birth("andy".to_string(), "matthew".to_string());
    t.birth("bob".to_string(), "alex".to_string());
    t.birth("bob".to_string(), "asha".to_string());
    let order = t.get_inheritance_order();
    assert_eq!(
        order,
        vec_string![
            "king",
            "andy",
            "matthew",
            "bob",
            "alex",
            "asha",
            "catherine"
        ]
    );
    t.death("bob".to_string());
    let order = t.get_inheritance_order();
    assert_eq!(
        order,
        vec_string!["king", "andy", "matthew", "alex", "asha", "catherine"]
    );
}
