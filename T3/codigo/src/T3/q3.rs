use graph_lib::graph::weighted_graph::Direction;

/*
[Colora¸c~ao de V´ertices] (2,5pts) Crie um programa que recebe um grafo n~ao-dirigido e n~ao-ponderado como
argumento. Ao final, informe a colora¸c~ao m´ınima e qual n´umero crom´atico foi utilizado em cada v´ertice. Use o
algoritmo de Lawler para cumprir essa quest~ao. Ao final, imprima na tela a quantidade de cores encontradas (na
primeira linha) e qual ´e a cor correspondente a cada v´ertice (segunda linha), como no exemplo de 6 v´ertices na
sa´ıda abaixo (v´ertice 1 tem a cor 2, v´ertice 2 tem a cor 1, v´ertice 3 tem a cor 2, v´ertice 4 tem a cor 1, v´ertice 5 tem
a cor 3 e v´ertice 6 tem a cor 2).
*/
//
use crate::graph::weighted_graph::WeightedGraph;
use crate::coloracao::coloracao::*;
use std::collections::HashSet;


// Função para mostrar a coloração. Não é otimizada como Lawler
pub fn colore(grafo: &WeightedGraph<String>) -> Vec<Option<usize>> {
    let mut cores: Vec<Option<usize>> = vec![None; grafo.qtdVertices() as usize];
    let mut possiveis: Vec<usize> = Vec::new();

    cores[0] = Some(1);
    possiveis.push(1);

    let mut i: usize = 2;
    for u in 0..grafo.qtdVertices() {
        let mut cores_vizinhos = Vec::new();
        for v in grafo.vizinhos(u, Direction::All) {
            if let Some(cor) = cores[v as usize - 1] {
                cores_vizinhos.push(cor);
            }
        }

        let cores_vizinhos_set: HashSet<usize> = cores_vizinhos.into_iter().collect();
        let cores_restantes: Vec<usize> = possiveis
            .iter()
            .cloned()
            .filter(|cor| !cores_vizinhos_set.contains(cor))
            .collect();

        if let Some(&cor) = cores_restantes.first() {
            cores[u as usize] = Some(cor);
        } else {
            cores[u as usize] = Some(i);
            possiveis.push(i);
            i += 1;
        }
    }

    cores
}

pub  fn q3(file_path: String){
    #![allow(non_snake_case)]
    let G = WeightedGraph::<String>::ler(&file_path, false);
    println!("{:?}", lawler(&G));

    for x in colore(&G){
        println!("{}", x.unwrap());
    }
    
}
