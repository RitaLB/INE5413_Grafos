mod graph; // Importa o módulo graph.rs
mod floyd_warshall; // Importa o módulo buscas.rs
// Importando o módulo `weighted_graph`
use crate::graph::weighted_graph::*;
use crate::floyd_warshall::floydWarshall::*;
use std::env;

#[allow(non_snake_case)]
fn main(){
    // Coletando dados da linha de comando
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_path = format!("src/grafos_teste/{}", filename);

    // Lendo o grafo
    let graph = WeightedGraph::<String>::ler(&file_path);

    // Resolvendo questão 5
    // Distâncias de todos para todos:
    let D = floyd_warshall(&graph);

    // Imprimir as distâncias referentes de cada vértice
    for (i, d) in D.iter().enumerate() {
        println!("{}:{}", i + 1, d.iter().map(|&x| x as i32).map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    }

}