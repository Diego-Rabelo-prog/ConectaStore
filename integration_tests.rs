use megastore::recommendation::{Product, RecommendationSystem};

#[test]
fn fluxo_completo_de_cadastro_compra_e_recomendacao() {
    let mut system = RecommendationSystem::new();

    system.add_product(Product::new(11, "Notebook", "Eletronicos", 2500.0)).unwrap();
    system.add_product(Product::new(12, "Mouse", "Acessorios", 80.0)).unwrap();
    system.add_product(Product::new(13, "Mochila", "Acessorios", 150.0)).unwrap();

    system.register_purchase(1, 11).unwrap();
    system.register_purchase(1, 12).unwrap();
    system.register_purchase(2, 11).unwrap();
    system.register_purchase(2, 13).unwrap();

    let recommendations = system.recommend_for_customer(1, 5);
    assert_eq!(recommendations.len(), 1);
    assert_eq!(recommendations[0].id, 13);
    assert_eq!(system.product_count(), 3);
    assert!(system.graph_node_count() >= 5);
    assert!(system.graph_edge_count() >= 4);
}
