pub mod q1{
    // Importando o módulo `weighted_graph`
    use crate::graph::weighted_graph::*;
    pub fn q1(filename: String){
        // Coletando dados da linha de comando
        let file_path = format!("src/grafos_teste/{}", filename);

        // Resolvendo questão 1: Lendo o grafo
        let graph: WeightedGraph<String> = WeightedGraph::<String>::ler(&file_path, false);

        // Printando testes das funções requisitadas
        
        // Teste qtdVertices e qtdArestas
        println!("Number of vertices: {}", graph.qtdVertices());
        println!("Number of edges: {}", graph.qtdArestas());

        // Teste grau
        println!("Degree of vertex '1': {}", graph.grau(1));

        // Teste rotulo
        println!("Rotulo of vertex '1': {}", graph.rotulo(1));

        // Teste vizinhos
        println!("Outgoing neighbors of '1': {:?}", graph.vizinhos(1, Direction::Outgoing));
        println!("Arriving neighbors of '1': {:?}", graph.vizinhos(1, Direction::Arriving));

        // Teste haAresta
        println!("There is an edge between '1' and '3': {}", graph.haAresta(1, 3));

        // Teste peso
        println!("Weight of edge between '1' and '3': {}", graph.peso(1, 3));

        // Testando a função find_edge_by_nodes
        match graph.find_edge_by_nodes(1, 3) {
            Some(edge) => println!("Peso da aresta entre '1' e '3': {}", edge.weight),
            None => println!("Aresta entre 'A' e 'B' não encontrada"),
        }

        println!("Edges:");
        for (vertex, edges) in graph.adj.iter() {
            for edge in edges {
                println!("{} -{}-> {}", vertex, edge.weight, edge.neighbor);
            }
        }

        println!("Nodes:");
        for node in &graph.nodes {
            println!("ID: {}, Rótulo: {}", node.id, node.rotulo);
        }
    }

}