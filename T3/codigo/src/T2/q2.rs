/*Crie um programa que receba um arquivo de grafo dirigido n~ao-ponderado com
v´ertices rotulados como argumento. O programa deve fazer executar uma Ordena¸c~ao Topol´ogica. Depois exiba a
ordem topol´ogica, utilizando os r´otulos de cada v´ertice, como no exemplo abaixo:*/

use crate::graph::weighted_graph::WeightedGraph;
use crate::ordenacao_topologica::ordenacao_topologica::ordenacao_topologica;

pub fn q2(file_path: String){
    #![allow(non_snake_case)]
    let graph = WeightedGraph::<String>::ler(&file_path, true);
    let O = ordenacao_topologica(&graph);

    for i in 0.. O.len()-1{
        let v = O[i];
        print!("{} -> ", graph.rotulo(v as i32));
    }
    print!("{} \n",graph.rotulo( O[O.len()-1] as i32) );
}