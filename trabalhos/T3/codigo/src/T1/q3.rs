pub mod q3{

    // Importando o módulo `weighted_graph`
    use crate::graph::weighted_graph::*;
    use crate::ciclo_euleriano::cicloEuleriano::*;
    use std::env;

    #[allow(non_snake_case)]
    pub fn q3( filename: String){
        // Coletando dados da linha de comando
        //let args: Vec<String> = env::args().collect();
        //let filename = &args[1];
        let file_path = format!("src/grafos_teste/{}", filename);

        // Resolvendo questão 3

        // Lendo o grafo
        let graph = WeightedGraph::<String>::ler(&file_path, false);

        // Verificando se o grafo possui um ciclo euleriano
        let (r, ciclo) = cicloEuleriano(&graph);
        if r{
            println!("1");
            for i in 0..ciclo.len() -1{
                print!("{},", ciclo[i]);
            }
            print!("{}", ciclo[ciclo.len()-1]);
            println!();
        }
        else{
            println!("não tem ciclo: 0");
        }
    }

}
