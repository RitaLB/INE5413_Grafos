/*
Crie um programa que recebe um grafo n~ao-dirigido e ponderado como argumento.
Ao final, o programa dever´a determinar qual a ´arvore geradora m´ınima. O programa dever´a imprimir o somat´orio
das arestas na ´arvore na primeira linha e as arestas que pertencem a ´arvore geradora m´ınima na segunda linha,
como no exemplo abaixo:
*/
use crate::graph::weighted_graph::WeightedGraph;
use crate::kruskal::kruskal::kruskal;

pub fn q3(file_path: String){
    use std::collections::HashSet;

    let graph = WeightedGraph::<String>::ler(&file_path, true);
    
    // Conjunto das arestas da árvore geradora mínima
    let a: HashSet<(i32, i32)> = kruskal(&graph);

    // Soma dos pesos das arestas da árvore geradora mínima
    let peso_total: f32 = a.iter().map(|&(u, v)|graph.peso(u, v)).sum();
    println!("{}", peso_total);

    // Imprimendo a resposta
    let arestas: Vec<String> = a.iter().map(|&(u, v, )| format!("{}-{}", u, v)).collect();
    println!("{}", arestas.join(", "));
}