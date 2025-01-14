pub mod fluxo_maximo{
    #![allow(non_snake_case)]
    #![allow(unused)]
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use crate::graph::weighted_graph::*;

    /*
    [Edmonds-Karp] (2,5pts) Crie um programa que receba um grafo dirigido e ponderado como argumento. Ao final,
    imprima na tela o valor do fluxo m´aximo resultante da execu¸c˜ao do algoritmo de Edmonds-Karp, como no exemplo
    abaixo.
    */

    pub fn bfs_edmons_karp(G: &WeightedGraph<String>, s: i32, t:i32, F: &Vec<Vec<f32>>) ->  Option<Vec<i32>> {

        let mut A = vec![-1; G.qtdVertices() as usize]; // A <- ∅ para todo v ∈ V
        A[s as usize -1] = s;

        // Iniciar busca pela fonte
        let mut Q = VecDeque::new(); // Q <- Fila()
        Q.push_back(s); // Q.enqueue(s)

        // propafação das visitas
        while !Q.is_empty() {
            //println!("entrou no while Q");
            // visitando o vértice u
            let u = Q.pop_front().unwrap(); // u <- Q.dequeue()
            let neighbors = G.vizinhos(u, Direction::Outgoing); // neighbors <- N+(u)
            for v in neighbors {
                //println!("entrou no for vizinhos");
                if (A[v as usize -1] == -1) && (G.peso(u,v) - F[u as usize -1][v as usize-1] > 0.0){ // if Cv == false ^ cf((u,v)) > 0 then 
                    // conhecendo o vértice neighbor (v)
                    A[v as usize-1] = u; // Aneighbor <- u
                    Q.push_back(v); // Q.enqueue(v)
                    // Sorvedouro encontrado. Criar caminho aumentante.
                    if v == t {
                        let mut p =Vec::new();
                        let mut w = v;
                        loop{ 
                            p.insert(0,w);
                            if  w == s{
                                break;
                            }
                            w = A[w as usize-1];
                        }
                        return Some(p);
                    }
                }
            }
        }
        return None;
    }

    pub fn edmons_karp(G: &WeightedGraph<String>, s: i32, t: i32)-> f32{
        let n: i32 = G.qtdVertices().clone();
        let mut F = vec![vec![0.0; n as usize]; n as usize];

        // Lembrar: Cf (u,v) = C(u,v) - F(u,v) =  w(u,v) - F[u][v]

        loop{
            // Encontrar um caminho aumentante
            let p = bfs_edmons_karp(G, s, t, &F);
            match p {
                Some(p) => {
                    // Encontrar fp = min{cf(u,v) | (u,v) ∈ p}
                    let mut fp = std::f32::INFINITY;
                    for i in 0..p.len()-1 {
                        let u = p[i];
                        let v = p[i+1];
                        let cf_uv = G.peso(u,v) - F[u as usize -1][v as usize -1];
                        if cf_uv < fp {
                            fp = cf_uv;
                        }
                    }
                    // Atualizar o fluxo
                    for i in 0..p.len()-1 {
                        let u = p[i];
                        let v = p[i+1];
                        F[u as usize-1][v as usize -1] = F[u as usize -1][v as usize -1] + fp;
                        F[v as usize -1][u as usize -1] = F[v as usize -1][u as usize -1] - fp;
                    }
                },
                None => {
                    break;
                }
            }
        }
        // Calcular o fluxo máximo
        let mut fluxo_maximo = 0.0;
        for i in 1..n+1 {
            fluxo_maximo = fluxo_maximo + F[s as usize -1][i as usize -1];
        }
        fluxo_maximo
    }


}