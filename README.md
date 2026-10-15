# ConectaStore — Sistema de Recomendação Baseado em Grafos

## 1. Objetivo
Demonstrar como um grafo pode relacionar clientes e produtos para sugerir itens que ainda não foram comprados por determinado cliente, usando históricos de compra compartilhados.

## 2. Tecnologias
- Rust 2021
- Biblioteca padrão do Rust; nenhuma dependência externa
- `HashMap` para consultar produtos e listas de adjacência
- `Vec` para armazenar vizinhos e resultados
- `VecDeque` e `HashSet` para BFS e controle de visitados/duplicatas
- `Instant` para medir tempo de execução

## 3. Modelo de dados
- **Vértices:** clientes (`Customer(id)`) e produtos (`Product(id)`).
- **Arestas:** uma compra conecta um cliente a um produto.
- **Peso:** número de vezes que a relação cliente-produto foi registrada.
- **Tipo:** grafo não direcionado e bipartido no modelo atual. Uma aresta sempre liga um cliente a um produto.
- **Representação:** lista de adjacência, implementada por `HashMap<Node, Vec<Edge>>`.

O sistema agrega compras repetidas na mesma aresta, incrementando seu peso, em vez de criar conexões duplicadas.

## 4. Algoritmo de recomendação
Para um cliente, o sistema identifica os produtos que ele já comprou. Em seguida, percorre relações cliente → produto compartilhado → outro cliente → produto candidato. Os candidatos já comprados são descartados, as pontuações são acumuladas e os resultados são ordenados pela pontuação decrescente. Empates são resolvidos pelo menor ID para tornar a saída determinística.

A busca em largura (BFS) também está implementada para demonstrar o percurso do grafo. A recomendação usa o padrão de caminhos de três arestas e pontua os candidatos a partir das relações compartilhadas. A BFS é útil para explorar níveis de conexão; a função de recomendação faz uma varredura direcionada dos vizinhos para calcular os escores.

## 5. Como executar
É necessário ter o Rust instalado (incluindo `cargo`).

```bash
cargo run
```

## 6. Como executar os testes
```bash
cargo test
```

Os testes unitários estão no módulo `src/recommendation.rs`. Os testes de integração estão em `tests/integration_tests.rs`.

## 7. Medição de desempenho
O programa executa uma medição simples de recomendação com grafos sintéticos de 100, 1.000 e 5.000 clientes, cada um com duas compras registradas. O tempo real depende do computador, sistema operacional e carga no momento da execução. Os valores devem ser copiados da execução real para o relatório e para a apresentação; não devem ser inventados.

## 8. Complexidade e escalabilidade
Considere `V` vértices, `E` arestas e `d(v)` o grau de um vértice.
- **Adicionar/consultar produto no `HashMap`:** O(1) esperado.
- **BFS:** O(V + E) no caso geral, com lista de adjacência.
- **Memória do grafo:** O(V + E).
- **Recomendação:** para o cliente consultado, o trabalho é proporcional à soma das conexões dos produtos comprados e dos clientes encontrados por essas conexões. Em forma aproximada, `O(Σ d(p) + Σ d(c))`, além da ordenação dos `K` candidatos em `O(K log K)`.
- **Lista versus matriz:** lista de adjacência tende a ser mais econômica em grafos esparsos, pois armazena vértices e conexões existentes. Matriz exige O(V²) posições, mesmo quando poucas conexões existem. Para um catálogo grande, a lista é a escolha mais adequada.

## 9. Limitações e melhorias futuras
Este protótipo usa dados em memória e compras fictícias. Em produção, seria necessário persistir os dados, autenticar acessos, tratar privacidade, atualizar o grafo a partir de eventos reais e avaliar qualidade das recomendações com métricas e testes com dados reais. Categorias, avaliações, disponibilidade e preço poderiam ser incorporados ao cálculo.

## 10. Arquitetura
- `src/main.rs`: cria catálogo de demonstração, registra compras, mostra recomendações e mede tempos.
- `src/recommendation.rs`: modelos, grafo, operações, recomendação e testes unitários.
- `tests/integration_tests.rs`: valida o fluxo completo.
- `Cargo.toml`: metadados e configuração do projeto.

## 11. Vídeo pitch
Inserir aqui o link público ou não listado do vídeo após a gravação.

## 12. Repositório
Inserir aqui o link do repositório público do GitHub após publicar o projeto.
