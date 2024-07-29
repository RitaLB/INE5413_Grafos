mod graph; // Importa o módulo graph.rs
//mod buscas; // Importa o módulo buscas.rs
// Importando o módulo `weighted_graph`
use crate::graph::weighted_graph::*;
//use crate::buscas::buscas::*;

fn main() {
    let filename = "/home/ritinha/Documents/faculdade/S4/grafos/t1/src/t1.txt".to_string();
    q1(&filename);
    //q2(&filename, 1);

}

fn q1(filename: &String){
    let graph: WeightedGraph<i32, String> = WeightedGraph::<i32, String>::ler(&filename);

    // Teste qtdVertices e qtdArestas
    println!("Number of vertices: {}", graph.qtdVertices());
    println!("Number of edges: {}", graph.qtdArestas());

    // Teste grau
    println!("Degree of vertex '1': {}", graph.grau(&1));

    // Teste rotulo
    println!("Rotulo of vertex '1': {}", graph.rotulo(&1));

    // Teste vizinhos
    println!("Outgoing neighbors of '1': {:?}", graph.vizinhos(1, Direction::Outgoing));
    println!("Arriving neighbors of '1': {:?}", graph.vizinhos(1, Direction::Arriving));

    // Teste haAresta
    println!("There is an edge between '1' and '3': {}", graph.haAresta(&1, &3));

    // Teste peso
    println!("Weight of edge between '1' and '3': {}", graph.peso(&1, &3));

    // Testando a função find_edge_by_nodes
    match graph.find_edge_by_nodes(&1, &3) {
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
    for (id, node) in &graph.nodes {
        println!("ID: {}, Rótulo: {}", id, node.rotulo);
    }
}

/* 
fn q2(filename: &String, s: i32){
    let graph = WeightedGraph::<i32, String>::ler(filename);
    let (D, A) = breadthFirstSearch(&graph, s);
    for i in 0..D.len() {
        println!("{}: {}", D[i], A[i]);
    }
}

*/