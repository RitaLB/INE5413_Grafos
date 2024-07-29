pub mod ordenacao_topologica{
    #![allow(non_snake_case)]
    use crate::graph::weighted_graph::*;

    pub fn ordenacao_topologica(graph: &WeightedGraph<String>) -> Vec<usize> {
        let mut C = vec![false; graph.qtdVertices() as usize]; // Cv <- false para todo v ∈ V
        let mut T: Vec<f32> = vec![f32::INFINITY; graph.qtdVertices() as usize];// Tv <- ∞ para todo v ∈ V
        let mut F: Vec<f32> = vec![f32::INFINITY; graph.qtdVertices() as usize];// Fv <- ∞ para todo v ∈ V

        // configurando o tempo de início
        let mut tempo = 0.0;

        // criando lista com os vértices ordenados topologicamente
        let mut O = vec![];

        for u in 1..graph.qtdVertices() +1 { // foreach u ∈ V do
            if !C[u as usize -1]{ // if Cu = false then
                dfs_visit_OT(graph, u, &mut C, &mut T, &mut F, &mut tempo, &mut O); 
            }
        }

        return O;
    }

    pub fn dfs_visit_OT(G: &WeightedGraph<String>, v : i32, C: &mut Vec<bool>, T: &mut Vec<f32>, F: &mut Vec<f32>, tempo: &mut f32, O: &mut Vec<usize>){
        C[v as usize-1] = true; // Cv <- true
        *tempo = *tempo +1.0; // tempo  <- tempo + 1
        T[v as usize-1] = *tempo; // Tv <- tempo

        for u in G.vizinhos(v, Direction::Outgoing){ // foreach u ∈ N+(v) do
            if C[u as usize-1] == false { // if Cu = false then
                dfs_visit_OT(G, u, C, T, F, tempo, O);
            }
        }
        *tempo += 1.0; // tempo <- tempo + 1
        F[v as usize -1] = *tempo; // Fv <- tempo
        // Adiciona o vértice v no início da lista O
        O.insert(0, v as usize);
    }
}