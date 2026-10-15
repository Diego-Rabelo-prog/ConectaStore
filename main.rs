mod recommendation;

use recommendation::{Graph, Product, RecommendationSystem};

fn main() {
    let mut system = RecommendationSystem::new();

    // Catalogo demonstrativo
    let products = vec![
        Product::new(101, "Fone Bluetooth", "Eletronicos", 149.90),
        Product::new(102, "Capa para celular", "Acessorios", 29.90),
        Product::new(103, "Carregador USB-C", "Acessorios", 59.90),
        Product::new(104, "Mouse sem fio", "Eletronicos", 89.90),
        Product::new(105, "Teclado mecanico", "Eletronicos", 199.90),
        Product::new(106, "Suporte para notebook", "Acessorios", 79.90),
    ];

    for product in products {
        if let Err(error) = system.add_product(product) {
            eprintln!("Erro ao cadastrar produto: {error}");
            return;
        }
    }

    // Relacoes representam compras de clientes. Repetir uma compra aumenta o peso da aresta.
    let purchases = [
        (1, 101), (1, 102), (1, 103),
        (2, 101), (2, 104), (2, 105),
        (3, 102), (3, 103), (3, 106),
        (4, 104), (4, 105), (4, 106),
        (2, 101), // segunda compra do produto 101 pelo cliente 2
    ];

    for (customer_id, product_id) in purchases {
        if let Err(error) = system.register_purchase(customer_id, product_id) {
            eprintln!("Erro ao registrar compra: {error}");
            return;
        }
    }

    println!("=== ConectaStore: recomendacao baseada em grafos ===");
    println!("Produtos cadastrados: {}", system.product_count());
    println!("Digite o identificador do cliente para receber recomendacoes.");
    println!("Exemplo: cliente 1");
    println!("Para demonstracao automatica, recomendamos para o cliente 1:\n");

    let recommendations = system.recommend_for_customer(1, 5);
    if recommendations.is_empty() {
        println!("Nenhuma recomendacao encontrada.");
    } else {
        for (position, product) in recommendations.iter().enumerate() {
            println!(
                "{}. {} (ID {}, categoria {}, R$ {:.2})",
                position + 1,
                product.name,
                product.id,
                product.category,
                product.price
            );
        }
    }

    println!("\nConsulta de produto por ID:");
    match system.get_product(104) {
        Some(product) => println!(
            "{} | categoria: {} | preco: R$ {:.2}",
            product.name, product.category, product.price
        ),
        None => println!("Produto nao encontrado."),
    }

    // Medicao simples de percurso em grafos sinteticos de tamanhos diferentes.
    println!("\n=== Medicao basica de desempenho ===");
    for customer_count in [100usize, 1_000, 5_000] {
        let mut benchmark = RecommendationSystem::new();
        for id in 1..=100u32 {
            let _ = benchmark.add_product(Product::new(
                id,
                format!("Produto {id}"),
                "Geral",
                10.0 + f64::from(id),
            ));
        }
        for customer in 1..=customer_count as u32 {
            let product_a = (customer % 100) + 1;
            let product_b = ((customer + 17) % 100) + 1;
            let _ = benchmark.register_purchase(customer, product_a);
            let _ = benchmark.register_purchase(customer, product_b);
        }
        let start = std::time::Instant::now();
        let _ = benchmark.recommend_for_customer(1, 10);
        let elapsed = start.elapsed();
        println!(
            "Clientes: {customer_count:>5} | tempo de recomendacao: {:?}",
            elapsed
        );
    }
}
