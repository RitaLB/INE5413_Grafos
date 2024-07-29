pub mod cicloEuleriano {
    #![allow(non_snake_case)]
    #![allow(unused)]
    use std::collections::HashMap;
    // Importando o módulo `weighted_graph`
    use std::collections::VecDeque;
    use std::vec;
    use crate::graph::weighted_graph::*;


    pub fn cicloEuleriano(graph: &WeightedGraph<String>) -> (bool, Vec<i32>) {
        let mut grau = 0;
    
        // Verificando se todos os vértices (nodos) possuem grau par
        for node in &graph.nodes {
            grau = graph.grau(node.id);
            if grau % 2 != 0 {
                return (false, vec![]);
            }
        }
    
        // Se todos os vértices possuem grau par, então o grafo possui um ciclo euleriano
        let ciclo = algoritmoHierholzer(graph);
        ciclo
    }
    
    fn algoritmoHierholzer(graph: &WeightedGraph<String>) -> (bool, Vec<i32>) {
        let arestas = graph.arestas();
        let mut E:  Vec<(i32, i32)> = arestas.iter().map(|(u, v, _)| (*u, *v)).collect();
        let n = graph.nodes.len();
        let mut C = vec![vec![true; n]; n]; 
    
        // Inicializar a matriz C com base nas arestas não conhecidas
        // C <- false para todo v ∈ E
        for &(u, v) in &E {
                C[u as usize-1][v as usize-1] = false;
                C[v as usize-1][u as usize-1] = false;
        }
    
        // v<-  selecionar um v pertencente a V arbitrariamente, que esteja conectado a uma aresta
        let mut v: i32 = 0;
        for node in &graph.nodes{
            if graph.grau(node.id) > 0{
                v = node.id;
                break;
            }
        }

        // Buscar subciclo euleriano
        let (r, ciclo) = buscarSubcicloEuleriano(graph, v, &mut C);
    
        if !r {
            return (false, vec![]);
        } else {
            let mut result = true;
            for row in &C {
                for &value in row {
                    result = result && value;
                }
            }
            if !result { // if ∃e ∈ E, Ce == false
                return (false, vec![]);
            } else {
                return (true, ciclo);
            }
        }
    }
    
    fn buscarSubcicloEuleriano(graph: &WeightedGraph<String>, v: i32, C: &mut Vec<Vec<bool>>) -> (bool, Vec<i32>) {
        
        let mut ciclo = Vec::new();
        ciclo.push(v); // ciclo <- (v)
        let t = v; // t <- v
        let mut vl = v;
    
        loop {
            let Nv = graph.vizinhos(vl, Direction::All);
            let mut aresta_selecionada = None;
            for u in Nv {
                if !C[vl as usize -1][u as usize -1]|| !C[u as usize -1][vl as usize -1]{
                    aresta_selecionada = Some((vl, u)); // {v,u} <- selecionar uma aresta e ∈ E tal que Ce == false
                    break;
                }
            }
            
            // Só prossegue se encontrar uma aresta não visitada conectada a Ciclo
            if let Some((v_atual, u)) = aresta_selecionada {
                C[v_atual as usize -1][u as usize -1] = true; // C{v,u} <- true
                C[u as usize -1][v_atual as usize -1] = true; // C{u,v} <- true (Como é um grafo não dirigido, a aresta é bidirecional)
                vl = u; // v <- u
                ciclo.push(vl); // Adiciona o vértice v ao final do ciclo; // ciclo <- ciclo.(v) 
            } else {
                return (false, vec![]);
            }
            if vl == t { // until v == t
                break;
            }
        }
        
        let mut i = 0;

        // Para todo vértice x no ciclo que tenha uma aresta adjacente não visitada.
        // foreach x ∈ { u ∈ ciclo : ∃{u,w} ∈ {e ∈ E: Ce = false}} do
        while i < ciclo.len() { // foreach x in ciclo 
            let xa = ciclo[i];
            let us = graph.vizinhos(xa, Direction::All); // foreach aresta {x, w} adjacente a x

            for u in us {

                if !C[xa as usize - 1][u as usize - 1] || !C[u as usize - 1][xa as usize - 1] { // if Ce{x, w} == false
                    let (r, ciclo_linha) = buscarSubcicloEuleriano(graph, xa, C);
                    if !r {
                        return (false, vec![]);
                    }
                    
                    ciclo.splice(i..i+1, ciclo_linha.into_iter());
                }
            }

            i += 1;
        }
        
        (true, ciclo)
    }
    

}