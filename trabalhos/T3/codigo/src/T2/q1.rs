/*
[Componentes Fortemente Conexas] (3,0pts) Crie um programa que receba um grafo dirigido e n˜ao-ponderado
como argumento. Ao final, imprima na tela as componentes fortemente conexas desse grafo. O exemplo abaixo
trata de uma sa´ıda v´alida, na qual identificou-se duas componentes fortemente conexas {3, 4, 5} e {1, 2, 6, 7}.
*/
use crate::graph::weighted_graph::WeightedGraph;
use crate::ComponenteFortementeConexa::ComponenteFortementeConexa::componentes_fortemente_conexas;
use crate::ComponenteFortementeConexa::ComponenteFortementeConexa::obter_componentes;

pub  fn q1(file_path: String){
    #![allow(non_snake_case)]
    // Coletando dados da linha de comando
    //let args: Vec<String> = env::args().collect();
    //let filename = &args[1];
    //let s = args[2].parse::<i32>().unwrap();


    // Lendo o grafo
    let graph = WeightedGraph::<String>::ler(&file_path, true);
    let Alt = componentes_fortemente_conexas(&graph);
    
    let componentes = obter_componentes(&Alt);
    for i in 0.. componentes.len() {
        let componente = &componentes[i as usize];
        for j in 0..componente.len()-1 {
            print!("{},", componente[j]);
        }
        print!("{}",componente[componente.len()-1]);
        println!(); 
    }
    
}
