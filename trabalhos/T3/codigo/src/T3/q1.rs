/*
 [Edmonds-Karp] (2,5pts) Crie um programa que receba um grafo dirigido e ponderado como argumento. Ao final,
imprima na tela o valor do fluxo m´aximo resultante da execu¸c˜ao do algoritmo de Edmonds-Karp, como no exemplo
abaixo.
*/
use crate::graph::weighted_graph::WeightedGraph;
use crate::edmons_karp::fluxo_maximo::edmons_karp;


pub  fn q1(file_path: String, s: i32, t: i32){
    #![allow(non_snake_case)]

    // Lendo o grafo
    let graph = WeightedGraph::<String>::ler(&file_path, true);
    let fluxo_max = edmons_karp(&graph, s, t);
    
    println!("{}", fluxo_max);
    
}
