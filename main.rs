mod component;
mod readfiles;
mod bfs;
type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
use crate::readfiles::Graph;

 pub fn main() {

  use readfiles::read_file;
  use component::count_components;
  use bfs::compute_average_distance_bfs;
  use bfs::subgraph;
  
  //read file and create graph base on dataset.
  read_file("facebook_combined.txt");
  let n = 4039;
  let mut edges: ListOfEdges = read_file("facebook_combined.txt");
  edges.sort();
  let graph = Graph::create_undirected(n,&edges);

  //find the most and least popular person
  let degrees = graph.get_all_degrees();
  let max_degree = degrees.iter().max();
  match max_degree {
      Some(max) => println!("The most popular person has {} degree", max),
      None => println!("The graph has no vertices"),
  }
  let min_degree = degrees.iter().min();
  match min_degree {
      Some(min) => println!("The most isolated person has {} degree", min),
      None => println!("The graph has no vertices"),
  }

  //find the average degree/average followers that one has
  let total_degree: usize = graph.get_all_degrees().iter().sum();
  let average_degree = total_degree as f64 / graph.n as f64;
  println!("Average degree: {:.2}", average_degree);

  //to find how many components does the data contain
  let num_components = count_components(&graph);
  println!("Number of components: {}", num_components);

  //by choosing random points for different amounts, we iterate each amount 15 times and find the
  //average distance for each one of them for finding a more stable pattern.
  let num_nodes_list = vec![100,200,300,500,1000,2000,3000,4000];
  let num_experiments = 15;
  for num_nodes in num_nodes_list {
      let mut total_distance = 0.0;
      for _ in 0..num_experiments {
          let subgraph = subgraph(&graph, num_nodes);
          let distance = compute_average_distance_bfs(&subgraph);
          //println!("experiement{}", distance);
          total_distance += distance;
      }
      let avg_distance = total_distance / num_experiments as f32;
      println!("Average distance in subgraph which has {} random nodes selected is: {}", num_nodes, avg_distance);
  }

  //finding the average distance between paris of vertices among the whole data set, not random
  let avg_distance = compute_average_distance_bfs(&graph);
  println!("The average distance between pairs of vertices among the whole data set is: {}", avg_distance);
  
}
