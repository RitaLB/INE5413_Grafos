pub mod q2 {
    // Importando o módulo `weighted_graph`
    use crate::graph::weighted_graph::*;
    use crate::breadth_first_search::breadthFirstSearch::*;
    use std::collections::HashMap;
    use std::env;
    #[allow(non_snake_case)]
    pub fn q2(filename: String, s: i32){
        // Coletando dados da linha de comando
        //let args: Vec<String> = env::args().collect();
        //let filename = &args[1];
        //let s = args[2].parse::<i32>().unwrap();
        let file_path = format!("src/grafos_teste/{}", filename);

        // Lendo o grafo
        let graph = WeightedGraph::<String>::ler(&file_path, false);
        
        // Resolvendo questão 2
        // Realizando a busca em largura
        let (D, _A) = breadthFirstSearch(&graph, s);

        // Imprimindo os níveis

        let mut levels: HashMap<i32, Vec<i32>> = HashMap::new();
        // Para cada distância, se a distância for diferente de infinito, adiciona o vértice ao nível correspondente
        for (i, &d) in D.iter().enumerate() {
            if d != std::i32::MAX {
                // Adiciona o vértice em seu respectivo nível
                levels.entry(d).or_insert(Vec::new()).push(i as i32 + 1);
            }
        }

        // Impressão sos níveis
        let mut sorted_levels: Vec<_> = levels.into_iter().collect();
        sorted_levels.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for (k, v) in sorted_levels {
            println!("{}: {}", k, v.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(","));
        }
    }
}