#[allow(non_snake_case)]
pub mod ComponenteFortementeConexa {
    use crate::graph::weighted_graph::*;

    pub fn componentes_fortemente_conexas(graph: &WeightedGraph<String>) -> Vec<i32> {
        // Chamar a DFS (do Algoritmo 16) para computar os tempos de término para cada vértice
        let ( _C,  _T,  _Al,  F) = dfs_cormen(graph); // Qual a utilidade de coletar C, T, Al se não são usadas?

        // Criar grafo transposto de G, chamado de GT
        let Gt = graph.build_grafo_transposto();

        // Chamar a DFS (do Algoritmo 16) alterado para que ele execute o laço da linha 6, selecionando vértices em ordem decrescente de F
        let ( _Ct, _Tt, Alt,  _Ft) = dfs_cormen_adaptado(&Gt, &F);

        return Alt;
    }


    pub fn dfs_cormen_adaptado(graph: &WeightedGraph<String>, F: &Vec<f32>) -> (Vec<bool>, Vec<f32>, Vec<i32>, Vec<f32>) {
        // Configurando todos os vértices
        let mut C = vec![false; graph.qtdVertices() as usize]; // Cv <- false para todo v ∈ V
        let mut T: Vec<f32> = vec![f32::INFINITY; graph.qtdVertices() as usize]; // Tv <- ∞ para todo v ∈ V
        let mut F_new: Vec<f32> = vec![f32::INFINITY; graph.qtdVertices() as usize]; // Fv <- ∞ para todo v ∈ V
        let valor_vazio = -1;
        let mut A = vec![valor_vazio; graph.qtdVertices() as usize]; // A <- ∅ para todo v ∈ V

        // Configurando o tempo de início
        let mut tempo = 0.0;

        // Ordenar os vértices em ordem decrescente de F
        let mut vertices_ordenados: Vec<_> = (1..F.len()+1).collect();
        //vertices_ordenados.sort_by(|&a, &b| F[b].partial_cmp(&F[a]).unwrap());
        ordenar_vertices_decrescente(&mut vertices_ordenados, &F);
        
        for u in vertices_ordenados { // foreach u ∈ V do
            if !C[u-1] { // if Cu = false then
                dfs_visit(graph, u as i32, &mut C, &mut T, &mut A, &mut F_new, &mut tempo);
            }
        }

        (C, T, A, F_new)
    }

    pub fn dfs_cormen(graph: &WeightedGraph<String>) -> (Vec<bool>, Vec<f32>, Vec<i32>, Vec<f32>) {
        // Configurando todos os vértices
        let mut C = vec![false; graph.qtdVertices() as usize]; // Cv <- false para todo v ∈ V
        let mut T: Vec<f32> = vec![f32::INFINITY; graph.qtdVertices() as usize]; // Tv <- ∞ para todo v ∈ V
        let mut F: Vec<f32> = vec![f32::INFINITY; graph.qtdVertices() as usize]; // Fv <- ∞ para todo v ∈ V
        let valor_vazio = -1;
        let mut A = vec![valor_vazio; graph.qtdVertices() as usize]; // A <- ∅ para todo v ∈ V

        // Configurando o tempo de início
        let mut tempo = 0.0;

        for u in 1..graph.qtdVertices()+1 { // foreach u ∈ V do 
            if !C[u as usize-1] { // if Cu = false then
                dfs_visit(graph, u as i32, &mut C, &mut T, &mut A, &mut F, &mut tempo);
            }
        }

        (C, T, A, F)
    }

    fn dfs_visit(
        G: &WeightedGraph<String>,
        v: i32,
        C: &mut Vec<bool>,
        T: &mut Vec<f32>,
        A: &mut Vec<i32>,
        F: &mut Vec<f32>,
        tempo: &mut f32,
    ) {
        C[v as usize-1] = true; // Cv <- true
        *tempo += 1.0; // tempo <- tempo + 1
        T[v as usize-1] = *tempo; // Tv <- tempo

        for u in G.vizinhos(v, Direction::Outgoing) { // foreach u ∈ N+(v) do
            if !C[u as usize-1] { // if Cu = false then
                A[u as usize-1] = v; // Au <- v
                dfs_visit(G, u, C, T, A, F, tempo);
            }
        }
        *tempo += 1.0;
        F[v as usize-1] = *tempo;
    }


    pub fn obter_componentes(Alt: &Vec<i32>) -> Vec<Vec<i32>> {
        let mut componentes = Vec::new();
        let mut visitado = vec![false; Alt.len()];
    
        for i in 1..Alt.len()+1 {
            if !visitado[i-1] {
                let mut componente = Vec::new();
                coleta_componentes(&Alt, i as i32, &mut visitado, &mut componente);
                componentes.push(componente);
            }
        }
    
        componentes
    }
    
    pub fn coleta_componentes(Alt: &Vec<i32>, v: i32, visitado: &mut Vec<bool>, componente: &mut Vec<i32>) {
        visitado[v as usize-1] = true;
        componente.push(v as i32 ); 
        for u in 1..Alt.len()+1 {
            if Alt[u-1] == v && !visitado[u-1] {
                coleta_componentes(Alt, u as i32, visitado, componente);
            }
        }
    }

    fn ordenar_vertices_decrescente(vertices: &mut Vec<usize>, t: &Vec<f32>) {
        let mut vertices_t: Vec<(_, _)> = vertices.iter().zip(t.iter()).map(|(&v, &valor_t)| (v, valor_t)).collect();
        
        vertices_t.sort_by(|&(_, valor_t1), &(_, valor_t2)| valor_t2.partial_cmp(&valor_t1).unwrap());
        
        *vertices = vertices_t.iter().map(|(v, _)| *v).collect();
    }
    
    
}

