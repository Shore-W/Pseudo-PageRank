pub(crate) mod graph;
//this pulls in our module "graph.rs"


fn main() {
    let graph = graph::DirectedGraph::new_graph("pagerank_data.txt");
    //creates a graph from our pagerank_data

    let pagerank_values = graph.compute_pagerank();
    //this computes the page rank value for each vertex in the graph

    let top_five = graph.find_top_5(&pagerank_values);
    //this sorts and reports the top 5 pagerank_values

    graph.print_results(&top_five);
    //this prints it out!
}

#[cfg(test)]
mod tests {
    use super::graph::*;
    fn test_5_graph() {
        let graph = DirectedGraph::new_graph("test_data.txt");
        //this is a graph that only has a single value, 5 

        let pagerank_values = graph.compute_pagerank();
        let top_five = graph.find_top_5(&pagerank_values);

        assert_eq!(graph.vertices, 5);
        //asserts that there are 5 vertices in our test file

        assert!(pagerank_values.iter().all(|&value| value == 0.0));
        //since there are no edges, the page rank must be 0.0

        assert!(top_five.is_empty());
        //since there are no page ranks, the top five should be empty
    }
}