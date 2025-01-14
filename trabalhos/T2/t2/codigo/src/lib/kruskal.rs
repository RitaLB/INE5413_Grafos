#![allow(non_snake_case)]
pub mod kruskal{
    use crate::graph::weighted_graph::*;
    use std::collections::HashSet; 

    pub fn kruskal(graph: &WeightedGraph<String>) -> HashSet<(i32, i32)> {
        // Conjunto das arestas
        let mut A: HashSet<(i32, i32)> = HashSet::new(); // A <- {}

        // Vetor que representa os conjuntos aos quais cada vértice é pertencente 
        let mut S: Vec<HashSet<i32>> = ((1..graph.qtdVertices() +1 )) // foreac v ∈ V do S[v] <- {v}
            .map(|v| vec![v].into_iter().collect())
            .collect();

        let mut El: Vec<(i32, i32, f32)> = graph // El <- lista de arestas ordenadas por ordem crescente de peso
            .arestas()
            .iter()
            .copied()
            .collect();
        El.sort_by(|&(u1, v1, _1), &(u2, v2, _)| graph.peso(u1, v1).partial_cmp(&graph.peso(u2, v2)).unwrap());

        // Montando o conjunto das arestas, passando por todas as arestas em ordem crescente de peso
        for &(u, v, _w) in &El {
            if S[u as usize -1] != S[v as usize - 1] { // if S[u] != S[v] then
                A.insert((u, v)); // A <- A ∪ {(u, v)}

                let x: HashSet<i32> = S[u as usize - 1].union(&S[v as usize - 1]).cloned().collect(); // x <- S[u] ∪ S[v]
                for &y in &x {
                    S[y as usize - 1] = x.clone(); // S[y] <- x para cada y ∈ x
                }
            }
        }
        A
    }
}