pub mod floydWarshall{
    #![allow(non_snake_case)]
    #![allow(unused)]
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use crate::graph::weighted_graph::*;

    // Algoritmo de Floyd-Warshall
    pub fn floyd_warshall(graph: &WeightedGraph<String>) -> Vec<Vec<f32>> {

        let mut  D = W(graph);
        // define um vertice K
        for k in 0..graph.qtdVertices() {
            // para cada aresta (U, V)
            for u in 0..graph.qtdVertices() {
                for v in 0..graph.qtdVertices() {
                    D[u as usize][v as usize] = D[u as usize][v as usize].min(D[u as usize][k as usize] + D[k as usize][v as usize]);
                }
            }
        }
    
        D
    }

    // Função W(graph)
    fn W(graph: &WeightedGraph<String>) -> Vec<Vec<f32>> {
        // Matriz que representa as distancias de todos os vertices para os demais vertices do grafo
        // seus valores:
        //   - 0               se   (i == j)
        //   - peso de (i,j)   se   (i != j) e (i,j) eh uma aresta existente
        //   - f32::INFINITY  se   (i != j) e (i,j) nao eh uma aresta existente
        let mut D: Vec<Vec<f32>> = vec![vec![0.0; graph.qtdVertices()as usize]; graph.qtdVertices() as usize];
        for i in 0..graph.qtdVertices() {
            for j in 0..graph.qtdVertices() {
                if i != j {
                    let peso = graph.peso(i as i32 + 1, j as i32 + 1);
                    D[i as usize][j as usize] = if peso as f32 != f32::INFINITY { peso as f32 } else { f32::INFINITY };
                }
            }
        }
        D
    }
    
}