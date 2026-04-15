use crate::product::{Catalog, Product};

pub struct SearchEngine {
    catalog: Catalog,
}

impl SearchEngine {

    pub fn new(catalog: Catalog) -> Self {
        SearchEngine { catalog }
    }

    pub fn search(&self, query: &str) -> Option<&Product> {
        self.catalog.get_product(query)
    }

}