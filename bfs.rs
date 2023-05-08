
use crate::readfiles::Graph;
type Vertex = usize;
use std::collections::VecDeque;
use rand::seq::SliceRandom;

//to compute the average distance from all distances collected
pub fn compute_average_distance_bfs(graph: &Graph) -> f32 {
    let mut total_distance = 0;
    let mut num_pairs = 0;
    for i in 0..graph.n {
        let distances = distance_bfs(i, graph);
        for j in i+1..graph.n {
            if let Some(distance) = distances[j] {
                total_distance += distance;
                num_pairs += 1;
            }
        }
    }
    return total_distance as f32 / num_pairs as f32;
}

//bfs algorithm for finding distances, returns a vector containing the distance of each vertex.
pub fn distance_bfs(start: Vertex, graph: &Graph) -> Vec<Option<u32>> {
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();

    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] {
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    return distance;
}

//to create subgraphs for randomly picking nodes for different amounts and find average distance afterwards
pub fn subgraph(graph: &Graph, num_nodes: usize) -> Graph {
    let nodes = (0..graph.n).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    let random_nodes = nodes.choose_multiple(&mut rng, num_nodes).cloned().collect::<Vec<_>>();
    let mut node_map = vec![None; graph.n];
    for (i, &node) in random_nodes.iter().enumerate() {
        node_map[node] = Some(i);
    }
    let mut edges = vec![];
    for u in &random_nodes {
        for &v in &graph.outedges[*u] {
            if let Some(i) = node_map[v] {
                edges.push((node_map[*u].unwrap(), i));
            }
        }
    }
    Graph::create_undirected(num_nodes, &edges)
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn t_compute_average_distance_bfs() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
        let graph = Graph::create_undirected(5, &edges);
        assert_relative_eq!(compute_average_distance_bfs(&graph), 1.5);
    }
}
