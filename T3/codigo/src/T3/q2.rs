use graph_lib::emparelhamento::emparelhamento::print_hopcroft_karp;

/*
[Hopcroft-Karp] (2,5pts) Crie um programa que receba um arquivo de grafo bipartido, n~ao-dirigido, n~ao-ponderado
e informe qual o valor do emparelhamento m´aximo e quais arestas pertencem a ele. Utilize o algoritmo de HopcroftKarp. Ao final, imprima na tela a quantidade de emparelhamentos encontrados (na primeira linha) e quais s~ao as
arestas correspondentes (na segunda linha), como no exemplo abaixo.
*/
use crate::graph::weighted_graph::WeightedGraph;
use crate::emparelhamento::emparelhamento::*;
fn criar_duplas(mate: &Vec<i32>) -> Vec<(i32, i32)> {
    let mut C = vec![false; mate.len()];
    C[0] = true;

    let mut A: Vec<(i32, i32)> = Vec::new();

    for u in 0..mate.len(){
        if mate[u] != 0{
            if !C[u]{
                C[u] = true;
                C[mate[u as usize] as usize] = true;
                A.push((u as i32, mate[u]));
            }
        }
    }
    return A;
}
pub fn q2(file_path: String){
    #![allow(non_snake_case)]

    // Lendo o grafo
    let graph = WeightedGraph::<String>::ler(&file_path, false);
    let (M, mate) = hopcroft_karp(&graph);

    let A = criar_duplas(&mate);
    println!("{}", M);
    for i in 0..A.len()-1 {
        let (u,v) = A[i];
        print!("{}-{},", u, v);
    }
    let (u,v) = A[A.len()-1];
    print!("{}-{}", u, v);
    println!();

    
}