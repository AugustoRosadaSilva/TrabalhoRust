mod product;
mod search;
mod graph;

use product::{Catalog, Product};
use search::SearchEngine;
use graph::Graph;

fn main() {

    let mut catalog = Catalog::new();

    catalog.add_product(Product {
        id: 1,
        name: "Notebook".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Dell".to_string(),
    });

    catalog.add_product(Product {
        id: 2,
        name: "Mouse".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Logitech".to_string(),
    });

    let search_engine = SearchEngine::new(catalog);

    if let Some(product) = search_engine.search("Mouse") {
        println!("Produto encontrado: {:?}", product);
    }

    let mut graph = Graph::new();

    graph.add_relation("Notebook", "Mouse");
    graph.add_relation("Notebook", "Teclado");

    if let Some(recommendations) = graph.recommendations("Notebook") {
        println!("Produtos recomendados: {:?}", recommendations);
    }
}