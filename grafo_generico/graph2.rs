// SIMPLIFICAR tipos T e U
pub mod weighted_graph{
    use std::fmt;
    use std::collections::HashMap;
    use std::fs::File;
    use std::hash::Hash;
    use std::io::{self, BufRead};
    use std::io::BufReader;

    pub struct Node<U> { 
        pub id: i32,
        pub rotulo: U,
    }
    pub struct Edge<T> { 
        pub neighbor: T, // Futuro: i32
        pub weight: f64,
    }
    pub struct WeightedGraph<T, U> {
        pub adj: HashMap<T, Vec<Edge<T>>>, // Futuro: HashMap<i32, Vec<Edge<T>>>
        pub directed: bool,
        pub nodes: HashMap<T, Node<U>>, // Futuro: vec![Node::new(); n] com cada nodo na posição correspondente ao seu id
    }

    pub enum Direction{
        Outgoing,
        Arriving,
        All,
    }

    impl<T: fmt::Display> Edge<T> {
        //Constructor, Time O(1) Space O(1)
        pub fn new(neighbor: T, weight: f64) -> Self {
            Edge { neighbor, weight }
        }
        
        
        // Format edge to enable it to be printed
        //Time O(1) Space O(1)
        pub fn to_string(&self) -> String {
            format!("({}, {})", self.neighbor, self.weight)
        }
    }


    impl<T: std::hash::Hash + Eq + Clone, U> WeightedGraph<T, U> {
        //Constructor, Time O(1) Space O(1)
        pub fn new(is_directed: bool) -> Self{
            WeightedGraph{
                adj: HashMap::new(),
                directed: is_directed,
                nodes:  HashMap::new(),
            }
        }
        
        // Cria um grafo NÃO DIRIGIDO a partir de um arquivo, Time O(n) Space O(n), n é número de arestas.
        pub fn newFromFile(&self , filename: &String) -> WeightedGraph<i32, String>{
            //let mut my_graph = WeightedGraph::new(false);
            let mut my_graph = WeightedGraph::<i32, String>::ler(filename);
            my_graph
        }


        pub fn add_edge(&mut self, a: T, b: T, w: f64){
            self.adj
                .entry(a.clone())
                .or_insert(Vec::<Edge<T>>::new())
                .push(Edge{ neighbor: b.clone(), weight: w});
            self.adj
                .entry(b.clone())
                .or_insert( Vec::<Edge<T>>::new());
            if !self.directed {
                self.adj
                    .entry(b.clone())
                    .or_insert(Vec::<Edge<T>>::new())
                    .push(Edge{ neighbor: a.clone(), weight: w});
            }
        }
        
        //Find the edge between two nodes, Time O(n) Space O(1), n is number of neighbors 
        pub fn find_edge_by_nodes(&self, a: &T, b: &T) -> Option<&Edge<T>> {
            if !self.adj.contains_key(a) || !self.adj.contains_key(b) {
                return None;
            }
        
            let ne = self.adj.get(a).unwrap();
            for edge in ne {
                if &edge.neighbor == b {
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
        pub fn grau(&self, v: &T) -> i32{
            match self.adj.get(v) {
                Some(edges) => {
                    return edges.len() as i32;
                },
                None => {
                    println!("This vertice does not exists");
                    0
                }
            }
        }

        // ############## rever
        pub fn rotulo(&self, node_id: &T) -> &U {
            let node:&Node<U> = self.nodes.get(&node_id).unwrap();
            &node.rotulo
        } 

        pub fn vizinhos(&self, v: &T, direction: Direction)-> Vec<&T> {
            let mut neighbors = Vec::<&T>::new();
            if !self.directed{
                // revisar questão das referências !!!!!!!!!!!
                match self.adj.get(v) {
                    Some(edges) => {
                       for edge in edges {
                            neighbors.push(&edge.neighbor)
                       }
                       return neighbors;
                    },
                    None => {
                        println!("This vertice does not exists");
                        return neighbors;
                    }
                }
            } else {
                match direction {
                    Direction::Outgoing => {
                        match self.adj.get(v) {
                            Some(edges) => {
                               for edge in edges {
                                    neighbors.push(&edge.neighbor)
                               }
                               return neighbors;
                            },
                            None => {
                                println!("This vertice does not exists");
                                return neighbors;
                            }
                        }
                    }
                    Direction::Arriving => {
                        for node in self.adj.keys(){
                            let edges = self.adj.get(node).unwrap();
                            for edge in edges {
                                if &edge.neighbor == v {
                                    neighbors.push(&edge.neighbor);
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

        pub fn haAresta(&self, u: &T, v: &T) -> bool{
            let edges = self.adj.get(u).unwrap();
            for edge in edges {
                if &edge.neighbor == v {
                    return true;
                }
            } 
            return false;
        }

        pub fn peso(&self, u: &T, v: &T) -> f64{
            let edges = self.adj.get(u).unwrap();
            for edge in edges {
                if &edge.neighbor == v {
                    return edge.weight;
                }
            } 
            std::f64::MAX
        } 
        

        //  Cria um grafo NÃO DIRIGIDO a partir de um arquivo, Time O(n) Space O(n), n é número de arestas.
        pub fn ler(filename: &String) -> WeightedGraph<i32, String>{
            println!("Reading file: {}", filename);
            let file = File::open(&filename).unwrap();
            let reader = io::BufReader::new(file);
            let mut estado = "inicial";
            let mut n_edges = 0;
            let mut nodes: HashMap<i32, Node<String>> = HashMap::new();
            let mut my_graph: WeightedGraph<i32, String> =  WeightedGraph::new(false);
            //let mut l = 0;
            
            for line in reader.lines() {
                let linha = line.unwrap();
                let tokens: Vec<&str> = linha.split_whitespace().collect();
                //println!("linha: {}", l);

                if estado == "nodes"{
                    estado = "reading_nodes";
                }
                if estado == "edges"{
                    estado = "reading_edges";
                }

                if tokens[0] == "*vertices" {
                    estado = "nodes";
                    if tokens.len() > 1 {
                        n_edges = tokens[1].parse::<i32>().unwrap();
                    }
                }

                if tokens[0] == "*edges" {
                    estado = "edges";
                    println!("estado edges");
                }

                if estado == "reading_nodes" {
                    let id = tokens[0].parse::<i32>().unwrap();
                    let rotulo = tokens[1].to_string();
                    nodes.insert(id, Node{id, rotulo});
                }

                if estado == "reading_edges" {
                    let id_a = tokens[0].parse::<i32>().unwrap();
                    let id_b = tokens[1].parse::<i32>().unwrap();
                    let w = tokens[2].parse().unwrap();
                    my_graph.add_edge(id_a, id_b, w);
                }
                //l += 1;
            }
            my_graph.nodes = nodes;
            my_graph
        }

    }
}