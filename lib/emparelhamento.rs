/*
[Hopcroft-Karp] (2,5pts) Crie um programa que receba um arquivo de grafo bipartido, n~ao-dirigido, n~ao-ponderado
e informe qual o valor do emparelhamento m´aximo e quais arestas pertencem a ele. Utilize o algoritmo de HopcroftKarp. Ao final, imprima na tela a quantidade de emparelhamentos encontrados (na primeira linha) e quais s~ao as
arestas correspondentes (na segunda linha), como no exemplo abaixo.
*/
pub mod emparelhamento{
    use std::collections::VecDeque;
    use crate::graph::weighted_graph::*;
    

    fn BFS_emparelhamento(G: &WeightedGraph<String>, mate: &mut Vec<i32>,D: &mut Vec<f32>)-> bool{
        let mut Q = VecDeque::new();

        // Iterando por todos os vertices de X. Se ele já está emparelhado, define distância 0 e adiciona na fila para busca
        // Se não, define distância infinita

        for (x, _, _) in G.arestas(){
            if mate[x as usize] == 0{
                D[x as usize] = 0.0;
                Q.push_back(x as i32);
            }else{
                D[x as usize] = std::f32::INFINITY;
            }
        }

        // Inicializa a distância do vértice fictício (sorvedouro virtual) D[0] como infinito
        D[0] = std::f32::INFINITY;

        // Enquanto existir vertices na fila
        while !Q.is_empty(){
            let x = Q.pop_front().unwrap(); // Proximo da fila

            // Vertice é relevante apenas se a distância dele até x for menor que a distancia até o sorvedouro virtual
            if D[x as usize] < D[0]{
                // Itera por todos os vizinhos de x
                for y in G.vizinhos(x, Direction::All){
                    if D[mate[y as usize ] as usize ] == std::f32::INFINITY{
                        D[mate[y as usize ] as usize ] = D[x as usize ] + 1.0;
                        Q.push_back(mate[y as usize ]);
                    }
                }
            }
        }

        // Se a distância do sorvedouro virtual for infinita, não existe caminho aumentante 
        return D[0] != std::f32::INFINITY;
    }   

    fn DFS_emparelhamento(G: &WeightedGraph<String>, mate: &mut Vec<i32>, D: &mut Vec<f32>, x: i32)-> bool{
        // Se não for o sorvedouro virtual
        if x != 0{
            for y in G.vizinhos(x, Direction::All){
                if D[mate[y as usize] as usize ] == D[x as usize ] + 1.0{
                    if DFS_emparelhamento(G, mate, D, mate[y as usize]){
                        mate[x as usize] = y;
                        mate[y as usize] = x;
                        return true;
                    }
                }
            }
            D[x as usize] = std::f32::INFINITY;
            return false;
        }
        return true;
    }

    pub fn hopcroft_karp(G: &WeightedGraph<String>)-> (i32, Vec<i32>){
        let n = G.qtdVertices();

        let mut D = vec![std::f32::INFINITY; n as usize + 1]; // Indica distância de um vértice 
        let mut mate = vec![0; n as usize + 1]; // Indica o emparelhamento de um vértice
       
        let mut M = 0; // Contador de emparelhamentos

        // Enquanto existir caminho aumentante
        while BFS_emparelhamento(G, &mut mate, &mut D){
            // Itera por todos os vértices de X e faz emparelhamento se possível
            for (x,_, _) in G.arestas(){
                if mate[x as usize] == 0 && DFS_emparelhamento(G, &mut mate, &mut D, x as i32){
                    M += 1;
                }
            }
        }
        
        return (M, mate);
    }

    
    pub fn print_hopcroft_karp(M: i32, mate: Vec<i32>){
        println!("{}", M);
        for (x, y) in mate.iter().enumerate(){
            if *y != 0 && x < *y as usize{
                println!("{}-{},", x, *y);
            }
        }
    }
    


}