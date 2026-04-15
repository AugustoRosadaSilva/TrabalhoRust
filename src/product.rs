use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub brand: String,
}

pub struct Catalog {
    pub products: HashMap<String, Product>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.name.clone(), product);
    }

    pub fn get_product(&self, name: &str) -> Option<&Product> {
        self.products.get(name)
    }
}