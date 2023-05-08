type Component = usize;
type Vertex = usize;
// type ListOfEdges = Vec<(Vertex,Vertex)>;
// type AdjacencyLists = Vec<Vec<Vertex>>;
use crate::readfiles::Graph;

//via bfs, starting from a given vertex and marks all the vertices in the same component 
pub fn mark_component_bfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
    component[vertex] = Some(component_no);
    
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(vertex);
    
    while let Some(v) = queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                component[*w] = Some(component_no);
                queue.push_back(*w);
            }
        }
    }
}

//counting the total number of components
pub fn count_components(graph: &Graph) -> usize {
    let mut component: Vec<Option<Component>> = vec![None; graph.n];
    let mut component_no: Component = 0;
    
    for v in 0..graph.n {
        if component[v].is_none() {
            mark_component_bfs(v, graph, &mut component, component_no);
            component_no += 1;
        }
    }
    
    component_no
}


#[test]
fn t_count_components() {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
    let graph = Graph::create_undirected(5, &edges);
    assert_eq!(count_components(&graph), 1);
    let edges = vec![(0, 1), (1, 2), (2, 0), (2, 3)];
    let graph = Graph::create_undirected(4, &edges);
    assert_eq!(count_components(&graph), 1);
    let edges = vec![(0, 1), (2, 3)];
    let graph = Graph::create_undirected(4, &edges);
    assert_eq!(count_components(&graph), 2);
}
