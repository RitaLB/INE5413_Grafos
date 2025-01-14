pub mod weighted_graph{
    #![allow(non_snake_case)]
    #![allow(unused)]
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead};

    #[derive(Clone)]
    pub struct Node<T> { 
        pub id: i32,
        pub rotulo: T,
    }
    pub struct Edge { 
        pub neighbor: i32, 
        pub weight: f32,
    }
    pub struct WeightedGraph<T> {
        pub adj: HashMap<i32, Vec<Edge>>, 
        pub directed: bool,
        pub nodes: Vec<Node<T>>, //Cada nodo na posição correspondente ao seu id
    }

    pub enum Direction{
        Outgoing,
        Arriving,
        All,
    }

    
    impl<T: Clone>  Node<T> {
        //Constructor, Time O(1) Space O(1)
        pub fn new(id: i32, rotulo: T) -> Self {
            Node { id, rotulo }
        }
    }

    impl Edge {
        //Constructor, Time O(1) Space O(1)
        pub fn new(neighbor: i32, weight: f32) -> Self {
            Edge { neighbor, weight }
        }
        
        
        // Format edge to enable it to be printed
        //Time O(1) Space O(1)
        pub fn to_string(&self) -> String {
            format!("({}, {})", self.neighbor, self.weight)
        }
    }


    impl<T: std::hash::Hash + Eq + Clone> WeightedGraph<T> {
        //Constructor, Time O(1) Space O(1)
        pub fn new(is_directed: bool) -> Self{
            WeightedGraph{
                adj: HashMap::new(),
                directed: is_directed,
                nodes:  Vec::new(),
            }
        }
        
        // Cria um grafo NÃO DIRIGIDO a partir de um arquivo, Time O(n) Space O(n), n é número de arestas.
        pub fn newFromFile(&self , filename: &String) -> WeightedGraph<String>{
            let mut my_graph = WeightedGraph::<String>::ler(filename, false);
            my_graph
        }


        pub fn add_edge(&mut self, a: i32, b: i32, w: f32){
            self.adj
                .entry(a.clone())
                .or_insert(Vec::<Edge>::new())
                .push(Edge{ neighbor: b.clone(), weight: w});
            self.adj
                .entry(b.clone())
                .or_insert( Vec::<Edge>::new());
            if !self.directed {
                self.adj
                    .entry(b.clone())
                    .or_insert(Vec::<Edge>::new())
                    .push(Edge{ neighbor: a.clone(), weight: w});
            }
        }
        
        //Find the edge between two nodes, Time O(n) Space O(1), n is number of neighbors 
        pub fn find_edge_by_nodes(&self, a: i32, b: i32) -> Option<&Edge> {
            if !self.adj.contains_key(&a) || !self.adj.contains_key(&b) {
                return None;
            }
        
            let ne = self.adj.get(&a).unwrap();
            for edge in ne {
                if edge.neighbor == b {
                    return Some(edge);
                }
            }
            None
        }

        // Returns the number of nodes (vertices) in the graph. 
        pub fn qtdVertices(&self)-> i32 {
            return self.adj.len() as i32;
        }

        // Returns number of edges
        pub fn qtdArestas(&self) -> i32 {
            let mut num_edges : i32 = 0;
            for (key, edges) in self.adj.iter() {
                num_edges += edges.len() as i32;
            }
            if !self.directed{
                num_edges = num_edges/2;
            }
            return num_edges;
        }

        // retorna o grau do vértice v;
        // Grau de um vértice: quantidade de arestas que se conectam a determinado vértice
        pub fn grau(&self, v: i32) -> i32{
            match self.adj.get(&v) {
                Some(edges) => {
                    return edges.len() as i32;
                },
                None => {
                    println!("This vertice does not exists A");
                    0
                }
            }
        }

        pub fn rotulo(&self, node_id: i32) -> &T {
            let node:&Node<T> = &self.nodes[node_id as usize -1];
            &node.rotulo
        } 

        pub fn vizinhos(&self, v: i32, direction: Direction)-> Vec<i32> {
            let mut neighbors = Vec::<i32>::new();
            if !self.directed{
                match self.adj.get(&v) {
                    Some(edges) => {
                       for edge in edges {
                            neighbors.push(edge.neighbor)
                       }
                       return neighbors;
                    },
                    None => {
                        println!("This vertice does not exists B");
                        return neighbors;
                    }
                }
            } else {
                match direction {
                    Direction::Outgoing => {
                        match self.adj.get(&v) {
                            Some(edges) => {
                               for edge in edges {
                                    neighbors.push(edge.neighbor)
                               }
                               return neighbors;
                            },
                            None => {
                                println!("This vertice does not exists C");
                                return neighbors;
                            }
                        }
                    }
                    Direction::Arriving => {
                        for node in self.adj.keys(){
                            let edges = self.adj.get(node).unwrap();
                            for edge in edges {
                                if edge.neighbor == v {
                                    neighbors.push(edge.neighbor);
                                }
                            }
                        }
                        return neighbors;
                    }
                    Direction::All => {
                        println!("Choose direction of neighbohod: Outgoing or Arriving. 'all' isn't a possible choice for directed graphs");
                        return neighbors;
                    }
                }
            }

        }

        pub fn haAresta(&self, u: i32, v: i32) -> bool{
            let edges = self.adj.get(&u).unwrap();
            for edge in edges {
                if edge.neighbor == v {
                    return true;
                }
            } 
            return false;
        }

        pub fn peso(&self, u: i32, v: i32) -> f32{
            let edges = self.adj.get(&u).unwrap();
            for edge in edges {
                if edge.neighbor == v {
                    return edge.weight;
                }
            } 
            std::f32::MAX
        } 

        pub fn mudar_peso(&mut self, u: i32, v: i32, w: f32){
            let edges = self.adj.get_mut(&u).unwrap();
            for edge in edges {
                if edge.neighbor == v {
                    edge.weight = w;
                }
            } 
        }
        

        //  Cria um grafo NÃO DIRIGIDO a partir de um arquivo, Time O(n) Space O(n), n é número de arestas.
        pub fn ler(filename: &String, dirigido: bool) -> WeightedGraph<String>{
            println!("Reading file: {}", filename);
            let file = File::open(&filename).unwrap();
            let reader = io::BufReader::new(file);
            let mut estado = "inicial";
            let mut n_nodes : i32= 0;
            let mut nodes = Vec::new();
            let mut my_graph: WeightedGraph<String> =  WeightedGraph::new(dirigido);
            let mut l = 0;
            
            for line in reader.lines() {
                let linha = line.unwrap();

                // Ignora linhas vazias
                if linha.trim().is_empty() {
                    continue;
                }

                let tokens: Vec<&str> = linha.split_whitespace().collect();

                if estado == "nodes"{
                    estado = "reading_nodes";
                }
                if estado == "edges"{
                    estado = "reading_edges";
                }

                if tokens[0] == "*vertices" {
                    estado = "nodes";
                    if tokens.len() > 1 {
                        n_nodes = tokens[1].parse::<i32>().unwrap();
                        let default_node = Node::new(-1, "default".to_string()); // Valor padrão para os nós
                        nodes = vec![default_node; n_nodes as usize];
                    }
                }
                
                if tokens[0] == "*edges" {
                    estado = "edges";
                }
                if tokens[0] == "*arcs"{
                    estado = "edges";
                }
                if estado == "reading_nodes" {
                    let id = tokens[0].parse::<i32>().unwrap();
                    let mut rotulo = String::new(); // Inicialize uma string vazia para o rótulo
                    
                    // Comece do segundo token até o final da linha
                    for i in 1..tokens.len() {
                        // Adicione cada token ao rótulo, separando por um espaço em branco, exceto para o primeiro token
                        if i > 1 {
                            rotulo.push_str(" "); // Adicione um espaço em branco antes de adicionar o próximo token
                        }
                        rotulo.push_str(tokens[i]); // Adicione o token ao rótulo
                    }
                    
                        // Remover as aspas do começo e do fim do rótulo, se presentes
                    if rotulo.starts_with('"') && rotulo.ends_with('"') {
                        rotulo = rotulo[1..rotulo.len() - 1].to_string();
                    }

                    // Agora, o rótulo contém toda a string entre as aspas
                    nodes[id as usize - 1] = Node { id, rotulo }; // Nodo na posição correspondente ao seu id-1
                }
                

                if estado == "reading_edges" {
                    let id_a = tokens[0].parse::<i32>().unwrap();
                    let id_b = tokens[1].parse::<i32>().unwrap();
                    let wf = tokens[2].parse::<f64>().unwrap();
                    let w = wf as f32;
                    my_graph.add_edge(id_a, id_b, w);
                }
                l += 1;
            }
            my_graph.nodes = nodes;
            my_graph
        }
    
        pub fn arestas(&self) -> Vec<(i32, i32, f32)> {
            let mut edges = Vec::new();
            
            for (node, neighbors) in self.adj.iter() {
                for neighbor in neighbors {
                    // Se o grafo for não direcionado, verifica se a origem é menor que o destino
                    // para evitar arestas duplicadas
                    if !self.directed && *node < neighbor.neighbor {
                        edges.push((node.clone(), neighbor.neighbor, neighbor.weight));
                    } else if self.directed {
                        edges.push((node.clone(), neighbor.neighbor, neighbor.weight));
                    }
                }
            }

            edges
        }

        pub fn build_grafo_transposto(&self) -> WeightedGraph<T>{
            let directed = self.directed;
            let mut graph_t: WeightedGraph<T> = WeightedGraph::new(directed);
            graph_t.nodes = self.nodes.clone();
            // criando transições transpostas
            for (chave, valor) in &self.adj {
                for edge in valor{
                    let num = edge.neighbor;
                    let w = edge.weight;
                    graph_t.add_edge(num.clone(), chave.clone(), w.clone())
                }
            }
            return graph_t;
        }

        // FUNÇÃO COM ERRO!!!
        pub fn build_rede_residual(&self) -> WeightedGraph<T>{
            let mut Gf = WeightedGraph::new(self.directed);
            Gf.nodes = self.nodes.clone();
            for (node, neighbors) in self.adj.iter() {
                for neighbor in neighbors {
                    let u = node.clone();
                    let v = neighbor.neighbor.clone();
                    let w_uv: f32 = neighbor.weight.clone();
                    Gf.add_edge(u, v, w_uv);
                    Gf.add_edge(v, u, 0.0);
                }
            }
            return Gf;
        }

        // Função para converter o grafo em uma matriz de adjacência
    pub fn to_adjacency_matrix(&self) -> Vec<Vec<f64>> {
        let n = self.nodes.len(); // número de nós
        let mut matrix = vec![vec![f64::INFINITY; n]; n]; // Inicializa a matriz com infinito

        // Itera sobre cada nó e suas arestas
        for (node_id, edges) in &self.adj {
            let u = *node_id as usize - 1; // Convertendo o ID do nó para índice (considerando que IDs começam de 1)
            for edge in edges {
                let v = edge.neighbor as usize - 1; // Convertendo o ID do vizinho para índice
                matrix[u][v] = edge.weight as f64; // Definindo o peso da aresta na matriz
                if !self.directed {
                    matrix[v][u] = edge.weight as f64; // Se o grafo não for dirigido, defina também a aresta oposta
                }
            }
        }

        matrix
    }
    }
}