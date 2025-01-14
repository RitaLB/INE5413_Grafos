/*
[Colora¸c~ao de V´ertices] (2,5pts) Crie um programa que recebe um grafo n~ao-dirigido e n~ao-ponderado como
argumento. Ao final, informe a colora¸c~ao m´ınima e qual n´umero crom´atico foi utilizado em cada v´ertice. Use o
algoritmo de Lawler para cumprir essa quest~ao. Ao final, imprima na tela a quantidade de cores encontradas (na
primeira linha) e qual ´e a cor correspondente a cada v´ertice (segunda linha), como no exemplo de 6 v´ertices na
sa´ıda abaixo (v´ertice 1 tem a cor 2, v´ertice 2 tem a cor 1, v´ertice 3 tem a cor 2, v´ertice 4 tem a cor 1, v´ertice 5 tem
a cor 3 e v´ertice 6 tem a cor 2).
*/


pub mod coloracao{
    use std::collections::HashMap;
    use crate::graph::weighted_graph::WeightedGraph;

    pub fn conjunto_potencia(itens: &Vec<i32>) -> Vec<Vec<i32>>{
        let n = itens.len();
        let mut potencia = Vec::new();
        for i in 0..(1 << n){
            let mut subconjunto = Vec::new();
            for j in 0..n{
                if i & (1 << j) != 0{
                    subconjunto.push(itens[j]);
                }
            }
            potencia.push(subconjunto);
        }
        return potencia;
    }

    pub fn conjuntos_indenpendentes(G: &WeightedGraph<String>, sl: Vec<i32>) -> Vec<Vec<i32>>{
        // Montando conjunto potÊncia dos vértices de G
        let mut S : Vec<Vec<i32>> = conjunto_potencia(&sl);

        // Lista de conjuntos independentes
        let mut R: Vec<Vec<i32>> = Vec::new();

        for s in S.clone().iter().skip(1) {
            let mut c = true;
            for v in s{
                for u in s{
                    if G.haAresta(*u, *v ){
                        c = false;
                        break;
                    }
                }
            }
            if c{
                let c_pot = conjunto_potencia(&s);
                for sc in c_pot{
                   // Ignora se o elemento não é encontrado
                    if let Some(pos) = S.iter().position(|x| *x == sc) {  
                        S.remove(pos);
                    }
                }
                R.push(s.to_vec());
            }
        }

        R
    }

    // Função principal
    pub fn lawler<T: Clone + std::hash::Hash + Eq>(graph: &WeightedGraph<T>) -> f32 {
        let num_vertices = graph.qtdVertices() as usize;
        // Lista para controlar o numero de cores
        let mut X = vec![f32::INFINITY; 1 << num_vertices]; // X <- vetor indexado entre 0 e 2^n - 1
        X[0] = 0.0; // X[0] <- 0

        let vertices: Vec<i32> = (1..(num_vertices+1) as i32).collect();
        let S = conjunto_potencia(&vertices); // Conjunto potencia dos vertices

        let mut f: HashMap<String, usize> = HashMap::new(); // # f(s) que mapeia os subconntos de 'S' a lista 'X'
        f.insert("[]".to_string(), 0);

        let arestas = graph.arestas();
        for (index, s) in S.iter().enumerate().skip(1) { //  foreach S pertencente a  S\{{}} do
            f.insert(format!("{:?}", s), index);

            // cria um novo grafo com os vértices do conjunto s
            let mut G_ = WeightedGraph::new(false); // G' <- (S, {u,v} | u,v pertencente a s e (u,v) pertence a E
            
            for &v in s {
                G_.adj.insert(v, Vec::new());
            }

            for i in 0..s.len() {
                for j in 0..s.len() { 
                    let u = s[i];
                    let v = s[j];
                    if s.contains(&(i as i32 +1)) && s.contains(&(j as i32 +1)) && arestas.contains(&(i as i32+1, j as i32 +1, 1.0)) || arestas.contains(&(j as i32 +1, i as i32+1, 1.0)){
                        G_.add_edge(u, v, 1.0);
                    }
                }
            }

            // Calculo para adicionar cor ou nao, utilizando o grafo G'
            for I in conjuntos_indenpendentes(&G_, s.clone()) { // foreach (u,v) pertencente a E do
                let diff = sorted_set_difference(&s, &I);
                let diff_str = format!("{:?}", diff);
                let index = f.get(&diff_str).unwrap();
                
                let i_str = format!("{:?}", I);
                let i = f.get(&i_str).unwrap(); 
                
                if X[*i] + 1.0 < X[*index] { // if Xi + 1 < Xindex then
                    X[*index] = X[*i] + 1.0; // Xindex <- Xi + 1
                }
            }
        }

        return X[(1 << num_vertices) - 1]; // return X[2^n - 1]
    }

    // Função auxiliar para calcular a diferença entre conjuntos
    fn sorted_set_difference(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
        let mut result = a.clone();
        for item in b {
            if let Some(pos) = result.iter().position(|x| x == item) {
                result.remove(pos);
            }
        }
        result.sort();
        result
    }
    


}