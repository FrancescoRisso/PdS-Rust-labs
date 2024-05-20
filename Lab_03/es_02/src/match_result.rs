use crate::node::Node;

pub struct MatchResult<'a> {
    q: &'a str,         // matched query string
    path: &'static str, // matched path
    node: &'a Node,     // matched node
}

impl<'a> MatchResult<'a> {
    pub fn new(query: &'a str, path: &'static str, node: &'a Node) -> Self {
        MatchResult {
            q: query,
            path,
            node,
        }
    }

    pub fn get_node(&self) -> &'a Node {
        self.node
    }

    pub fn get_path(&self) -> &str {
        self.path
    }

    pub fn get_query(&self) -> &'a str {
        self.q
    }
}
