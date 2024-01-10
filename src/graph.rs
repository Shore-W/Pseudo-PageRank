use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use rand::prelude::SliceRandom;

// this creates a struct that represents a directed graph with vertices and edges.
pub struct DirectedGraph {
    pub vertices: usize,
    pub edges: HashMap<usize, Vec<usize>>,
}

impl DirectedGraph {
    // Creates a new graph by reading data from a file.
    // The file should contain information about vertices and edges.
    // The first line of the file specifies the number of vertices.
    // Each subsequent line represents an edge between two vertices.
   
    // A new `DirectedGraph` instance.
    pub fn new_graph(filename: &str) -> DirectedGraph {
        let mut graph = DirectedGraph {
            vertices: 0,
            edges: HashMap::new(),
        };
        //this sets our graph to be a blank slate with 0 vertices and no edges

        if let Ok(file) = File::open(filename) {
            let reader = io::BufReader::new(file);

            let mut is_first_line = true;
            for line in reader.lines() {
                //for every line in our file...

                if let Ok(edge) = line {
                    //if there is an edge connected to the vertex...

                    if is_first_line {
                        //if the first line is just a number, then we set its graph vertices to be this first line.
                        graph.vertices = edge.trim().parse().unwrap_or(0);
                        //this trims it so it understands how many vertexs there are
                        is_first_line = false;
                        //now our first_line resets to false so that it doesn't try to find a new vertex count

                    } else {
                        let vertices: Vec<usize> = edge
                            .split_whitespace()
                            .filter_map(|s| s.parse().ok())
                            .collect();
                        //this parses through the line of txt in our data file and attempts to pull out a usize integer from the pile
                        //if the .parse() returns a "Some(value)" then it gets collected into vertices 

                        if let [from, to] = &vertices[..] {
                            graph.add_edge(*from, *to);
                            //this adds to the edge to our graph
                        }
                    }
                }
            }
        }

        graph
        //this returns our graph as the output of the funciton
    }

    // Adds an edge to the graph.
    pub fn add_edge(&mut self, from: usize, to: usize) {
    // The input for this function is a mutable reference to itself (self) and two usize variables: from and to.

    if from < self.vertices && to < self.vertices {
        // If both nodes are less than the total number of vertices in the graph...

        let entry = self.edges.entry(from).or_insert(Vec::new());
        // We use the entry method of the edges HashMap to get a mutable reference to the entry corresponding to the 'from' vertex.
        // If the entry does not exist, we insert an empty vector as the default value.

        entry.push(to);
        // We add the 'to' vertex to the vector associated with the 'from' vertex.
        // This represents the creation of a directed edge from 'from' to 'to'.
    }
}

    // Computes an approximate PageRank for each vertex in the graph.
    // The algorithm runs for 100 iterations, and for each iteration, it randomly selects neighbors based on a probability of 0.9 and updates the PageRank values.
    pub fn pagerank(&self) -> Vec<f64> {
        let n = self.vertices;
        let mut pagerank_values = vec![0.0; n];

        for start_vertex in 0..n {
            for _ in 0..100 {
                //initialzes our 100 step walk from the starting vertex...

                let mut current_vertex = start_vertex;
                for _ in 0..100 {
                    //initializes our 100 step walk from the current vertex
                    let mut rng = rand::thread_rng();
                    let random_value: f64 = rng.gen();

                    if let Some(neighbors) = self.edges.get(&current_vertex) {
                        // If the current vertex has neighbors...

                        if !neighbors.is_empty() && random_value < 0.9 {
                            current_vertex = *neighbors.choose(&mut rng).unwrap();
                            // With 90% probability, move to a random neighbor.

                        } else {
                            current_vertex = rng.gen_range(0..n);
                            // With 10% probability or if there are no neighbors, jump to a random vertex in the graph.

                        }
                    } else {
                        current_vertex = rng.gen_range(0..n);
                        // If the current vertex has no neighbors, jump to a random vertex in the graph.
                    }
                }

                pagerank_values[current_vertex] += 1.0;
                // Increment the PageRank value for the current vertex.
            }
        }

        for value in pagerank_values.iter_mut() {
            *value /= 100.0 * n as f64;
        }

        pagerank_values
    }

    // Finds the top five vertices based on their PageRank values.
    // A vector of tuples representing the top five vertices and their corresponding PageRank values.
    
    pub fn find_top_5(&self, pagerank_values: &[f64]) -> Vec<(usize, f64)> {
        let mut indices: Vec<usize> = (0..pagerank_values.len()).collect();
        indices.sort_by(|a, b| pagerank_values[*b].partial_cmp(&pagerank_values[*a]).unwrap());
        // The function takes a reference to the graph (self) and a slice of PageRank values.
        // It returns a vector of tuples representing the top five vertices and their PageRank values.


        indices.iter().take(5).map(|&i| (i, pagerank_values[i])).collect()
        // Sort the indices based on the corresponding PageRank values in descending order, and then returns the top 5 of the new list
    }

    // Prints the results, showing the top five vertices and their approximate PageRank values
    pub fn print_results(&self, top_five: &[(usize, f64)]) {
        for (vertex, rank) in top_five {
            println!("vertex {}: approximate PageRank {}", vertex, rank);
        }
    }
}