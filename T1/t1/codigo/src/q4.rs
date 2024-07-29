mod graph; // Importa o módulo graph.rs
mod bellman_ford; // Importa o módulo buscas.rs
// Importando o módulo `weighted_graph`
use crate::graph::weighted_graph::*;
use crate::bellman_ford::bellmanFord::*;
use std::env;


#[allow(non_snake_case)]
fn main (){
    // Coletando dados da linha de comando
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let s = args[2].parse::<i32>().unwrap();
    let file_path = format!("src/grafos_teste/{}", filename);

    // Lendo o grafo
    let graph = WeightedGraph::<String>::ler(&file_path);

    // Resolvendo questão 4
    let (existe, D, A) = bellman_ford(&graph, s);
    if existe{
        imprime_caminhos(graph, 1, &D, &A);
    }
    else{
        println!("Ciclo negativo encontrado");
    }
}

// Função para facilitar a impressão
#[allow(non_snake_case)]
fn imprime_caminhos(graph: WeightedGraph<String>, s: i32, D: &Vec<f32>, A: &Vec<i32>) {
    let mut path: Vec<i32> = Vec::new();
    for i in 1..=graph.qtdVertices() {
        path.clear();
        let mut a = i;
        while a != s {
            path.insert(0, a);
            a = A[(a - 1) as usize];
        }
        path.insert(0, s);
        print!("{}: ", i);
        for (j, &p) in path.iter().enumerate() {
            if j == path.len() - 1 {
                print!("{}", p);
            } else {
                print!("{},", p);
            }
        }
        print!("; d={}\n", D[(i - 1) as usize]);
    }
}