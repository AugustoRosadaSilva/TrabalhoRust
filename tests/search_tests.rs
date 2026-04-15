use megastore_search::product::{Catalog, Product};
use megastore_search::search::SearchEngine;

#[test]
fn test_product_search() {

    let mut catalog = Catalog::new();

    catalog.add_product(Product {
        id: 1,
        name: "Notebook".to_string(),
        category: "Eletronicos".to_string(),
        brand: "Dell".to_string(),
    });

    let search_engine = SearchEngine::new(catalog);

    let result = search_engine.search("Notebook");

    assert!(result.is_some());
}