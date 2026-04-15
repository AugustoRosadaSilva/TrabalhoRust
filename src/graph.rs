use std::collections::HashMap;

pub struct Graph {
    pub edges: HashMap<String, Vec<String>>,
}

impl Graph {

    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_relation(&mut self, product: &str, related: &str) {
        self.edges
            .entry(product.to_string())
            .or_insert(Vec::new())
            .push(related.to_string());
    }

    pub fn recommendations(&self, product: &str) -> Option<&Vec<String>> {
        self.edges.get(product)
    }

}