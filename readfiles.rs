use std::fs::File;
use std::io::prelude::*;
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

pub fn read_file(path: &str) -> Vec<(Vertex, Vertex)> {
    let mut result: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        if v.len() != 1{
            let z = v[1].parse::<Vertex>().unwrap();
            let x = v[0].parse::<Vertex>().unwrap();
            result.push((x, z));
        }else{
            continue;
        }
    }
    return  result;
}

#[derive(Debug)]
pub struct Graph {
    pub n: usize, 
    pub outedges: AdjacencyLists,
}

fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    pub fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    pub fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }

    pub fn create_undirected(n:usize,edges:&ListOfEdges)-> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g
    }
    pub fn get_out_degree(&self, u: Vertex) -> usize {
        self.outedges[u].len()
    }

    pub fn get_all_degrees(&self) -> Vec<usize> {
        (0..self.n).map(|u| self.get_out_degree(u)).collect()
    }
    

}



#[test]
fn t_create_undirected() {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
    let graph = Graph::create_undirected(4, &edges);
    assert_eq!(graph.get_out_degree(0), 2);
    assert_eq!(graph.get_out_degree(1), 2);
    assert_eq!(graph.get_out_degree(2), 2);
    assert_eq!(graph.get_out_degree(3), 2);
}
