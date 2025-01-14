pub mod bellmanFord{
    #![allow(non_snake_case)]
    #![allow(unused)]
    use std::collections::HashMap;
    // Importando o módulo `weighted_graph`
    use std::collections::VecDeque;
    use crate::graph::weighted_graph::*;

    // Algoritmo de Bellman-Ford
    pub fn bellman_ford(graph: &WeightedGraph<String>, s: i32) -> (bool, Vec<f32>, Vec<i32>){

        // inicialização
        let mut D: Vec<f32> = vec![f32::INFINITY; graph.qtdVertices() as usize];// Dv <- ∞ para todo v ∈ V
        let valor_vazio = -1;
        let mut A = vec![valor_vazio; graph.qtdVertices() as usize];// A <- ∅ para todo v ∈ V

        // Configurando o vértice de origem
        D[s as usize-1] = 0.0; // Ds <- 0

        
        for _ in 1..graph.qtdVertices()-1 {
            for (u, v, w) in graph.arestas() {
                // Relaxamento
                if D[v as usize-1] > D[u as usize-1] + w as f32 {
                    D[v as usize-1] = D[u as usize-1] + w as f32;
                    A[v as usize-1] = u;
                }
            }
        }

        // Verificação de ciclos negativos
        for (u, v, w) in graph.arestas() {
            if D[v as usize-1] > D[u as usize-1] + w as f32{
                return (false, vec![], vec![]);
            }
        }

        return(true, D,A)
    }
}