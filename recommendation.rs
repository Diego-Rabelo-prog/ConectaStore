use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: u32, name: impl Into<String>, category: impl Into<String>, price: f64) -> Self {
        Self {
            id,
            name: name.into(),
            category: category.into(),
            price,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Customer(u32),
    Product(u32),
}

#[derive(Debug, Clone)]
struct Edge {
    node: Node,
    weight: u32,
}

#[derive(Debug, Default)]
pub struct Graph {
    adjacency: HashMap<Node, Vec<Edge>>,
}

impl Graph {
    pub fn add_node(&mut self, node: Node) {
        self.adjacency.entry(node).or_default();
    }

    pub fn add_undirected_edge(&mut self, from: Node, to: Node) {
        self.add_weighted_purchase_edge(from, to);
    }

    fn add_weighted_purchase_edge(&mut self, customer: Node, product: Node) {
        self.add_node(customer.clone());
        self.add_node(product.clone());

        // A aresta de compra e bidirecional para permitir percorrer o grafo nos dois sentidos.
        Self::increment_edge(self.adjacency.get_mut(&customer).unwrap(), product.clone());
        Self::increment_edge(self.adjacency.get_mut(&product).unwrap(), customer);
    }

    fn increment_edge(edges: &mut Vec<Edge>, target: Node) {
        if let Some(edge) = edges.iter_mut().find(|edge| edge.node == target) {
            edge.weight += 1;
        } else {
            edges.push(Edge { node: target, weight: 1 });
        }
    }

    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(Vec::len).sum::<usize>() / 2
    }

    pub fn breadth_first_search(&self, start: &Node) -> Vec<(Node, usize)> {
        if !self.adjacency.contains_key(start) {
            return Vec::new();
        }

        let mut visited: HashSet<Node> = HashSet::new();
        let mut queue: VecDeque<(Node, usize)> = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start.clone());
        queue.push_back((start.clone(), 0));

        while let Some((current, depth)) = queue.pop_front() {
            result.push((current.clone(), depth));
            if let Some(neighbors) = self.adjacency.get(&current) {
                for edge in neighbors {
                    // O peso registra quantas vezes a relacao de compra ocorreu.
                    // BFS usa a conectividade; a contagem fica disponivel para evolucoes futuras.
                    let _purchase_strength = edge.weight;
                    if visited.insert(edge.node.clone()) {
                        queue.push_back((edge.node.clone(), depth + 1));
                    }
                }
            }
        }
        result
    }

    fn neighbors(&self, node: &Node) -> &[Edge] {
        self.adjacency.get(node).map(Vec::as_slice).unwrap_or(&[])
    }
}

#[derive(Debug, Default)]
pub struct RecommendationSystem {
    products: HashMap<u32, Product>,
    graph: Graph,
}

impl RecommendationSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), String> {
        if product.name.trim().is_empty() {
            return Err("O nome do produto nao pode ficar vazio.".to_string());
        }
        if !product.price.is_finite() || product.price < 0.0 {
            return Err("O preco deve ser um numero valido e nao negativo.".to_string());
        }
        if self.products.contains_key(&product.id) {
            return Err(format!("Ja existe produto com ID {}.", product.id));
        }

        self.graph.add_node(Node::Product(product.id));
        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn product_count(&self) -> usize {
        self.products.len()
    }

    pub fn register_purchase(&mut self, customer_id: u32, product_id: u32) -> Result<(), String> {
        if !self.products.contains_key(&product_id) {
            return Err(format!("Produto com ID {product_id} nao cadastrado."));
        }
        self.graph.add_weighted_purchase_edge(
            Node::Customer(customer_id),
            Node::Product(product_id),
        );
        Ok(())
    }

    pub fn recommend_for_customer(&self, customer_id: u32, limit: usize) -> Vec<&Product> {
        if limit == 0 {
            return Vec::new();
        }
        let start = Node::Customer(customer_id);
        if !self.graph.adjacency.contains_key(&start) {
            return Vec::new();
        }

        let already_bought: HashSet<u32> = self.graph
            .neighbors(&start)
            .iter()
            .filter_map(|edge| match &edge.node {
                Node::Product(id) => Some(*id),
                Node::Customer(_) => None,
            })
            .collect();

        // Cada caminho cliente -> produto -> outro cliente -> produto
        // sugere um item comprado por alguem com historico parcialmente semelhante.
        let mut scores: HashMap<u32, u32> = HashMap::new();
        for first_edge in self.graph.neighbors(&start) {
            let shared_product = match &first_edge.node {
                Node::Product(id) => *id,
                Node::Customer(_) => continue,
            };
            for customer_edge in self.graph.neighbors(&Node::Product(shared_product)) {
                let other_customer = match &customer_edge.node {
                    Node::Customer(id) if *id != customer_id => *id,
                    _ => continue,
                };
                for candidate_edge in self.graph.neighbors(&Node::Customer(other_customer)) {
                    if let Node::Product(candidate_id) = &candidate_edge.node {
                        if !already_bought.contains(candidate_id) {
                            *scores.entry(*candidate_id).or_insert(0) += customer_edge.weight;
                        }
                    }
                }
            }
        }

        // Ordenacao deterministica: maior pontuacao primeiro; em empate, menor ID.
        let mut ranked: Vec<(u32, u32)> = scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        ranked.into_iter()
            .take(limit)
            .filter_map(|(id, _score)| self.products.get(&id))
            .collect()
    }

    pub fn graph_node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn graph_edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn bfs_from_customer(&self, customer_id: u32) -> Vec<(Node, usize)> {
        self.graph.breadth_first_search(&Node::Customer(customer_id))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn sample_system() -> RecommendationSystem {
        let mut system = RecommendationSystem::new();
        for p in [
            Product::new(1, "Fone", "Eletronicos", 100.0),
            Product::new(2, "Capa", "Acessorios", 20.0),
            Product::new(3, "Carregador", "Acessorios", 30.0),
        ] {
            system.add_product(p).unwrap();
        }
        system.register_purchase(10, 1).unwrap();
        system.register_purchase(10, 2).unwrap();
        system.register_purchase(20, 1).unwrap();
        system.register_purchase(20, 3).unwrap();
        system
    }

    #[test]
    fn cadastra_e_consulta_produto() {
        let system = sample_system();
        assert_eq!(system.get_product(1).unwrap().name, "Fone");
        assert!(system.get_product(99).is_none());
    }

    #[test]
    fn rejeita_id_duplicado() {
        let mut system = RecommendationSystem::new();
        system.add_product(Product::new(1, "Fone", "Eletronicos", 100.0)).unwrap();
        assert!(system.add_product(Product::new(1, "Outro", "Geral", 10.0)).is_err());
    }

    #[test]
    fn rejeita_preco_negativo() {
        let mut system = RecommendationSystem::new();
        assert!(system.add_product(Product::new(1, "Produto", "Geral", -1.0)).is_err());
    }

    #[test]
    fn rejeita_compra_de_produto_inexistente() {
        let mut system = RecommendationSystem::new();
        assert!(system.register_purchase(1, 404).is_err());
    }

    #[test]
    fn recomenda_item_de_cliente_com_interesse_semelhante() {
        let system = sample_system();
        let result = system.recommend_for_customer(10, 10);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 3);
    }

    #[test]
    fn nao_recomenda_produto_ja_comprado_nem_duplica() {
        let mut system = sample_system();
        system.register_purchase(20, 3).unwrap();
        let result = system.recommend_for_customer(10, 10);
        let ids: HashSet<u32> = result.iter().map(|product| product.id).collect();
        assert!(!ids.contains(&1));
        assert!(!ids.contains(&2));
        assert_eq!(ids.len(), result.len());
    }

    #[test]
    fn bfs_retorna_o_cliente_de_partida() {
        let system = sample_system();
        let visited = system.bfs_from_customer(10);
        assert!(!visited.is_empty());
        assert_eq!(visited[0], (Node::Customer(10), 0));
    }
}
