pub mod breadthFirstSearch{
    #![allow(non_snake_case)]
    #![allow(unused)]
    use std::collections::HashMap;
    // Importando o módulo `weighted_graph`
    use std::collections::VecDeque;
    use crate::graph::weighted_graph::*;

    // WeightedGraph<i32, String>
    pub fn breadthFirstSearch(graph: &WeightedGraph< String>, s: i32) -> (Vec<i32>, Vec<i32>){
        // Configurando todos os vértices
        let mut C = vec![false; graph.qtdVertices() as usize]; // Cv <- false para todo v ∈ V
        let mut D = vec![std::i32::MAX; graph.qtdVertices() as usize]; // Dv <- ∞ para todo v ∈ V
        let valor_vazio = -1;
        let mut A = vec![valor_vazio; graph.qtdVertices() as usize];// A <- ∅ para todo v ∈ V

        // Configurando o vértice de origem
        C[s as usize-1] = true; // Cs <- true
        D[s as usize-1] = 0; // Ds <- 0

        // Preparando fila de visitas
        let mut Q = VecDeque::new(); // Q <- Fila()
        Q.push_back(s); // Q.enqueue(s)

        // Propagaçã das visitas
        while !Q.is_empty() {
            // Visitando o vértice u
            let u = Q.pop_front().unwrap(); // u <- Q.dequeue()
            let neighbors = graph.vizinhos(u, Direction::All); // neighbors <- N(u)
            for neighbor in neighbors {
                if !C[neighbor as usize-1] { // if Cneighbor == false
                    // Conhecendo o vértice neighbor (v)
                    C[neighbor as usize-1] = true; // Cneighbor <- true
                    D[neighbor as usize-1] = D[u as usize-1] + 1; // Dneighbor <- Dv + 1
                    A[neighbor as usize-1] = u; // Aneighbor <- u
                    Q.push_back(neighbor); // Q.enqueue(neighbor)
                }
            }
        }
        return(D,A)
    }
}
