/* CSE 381 - Dijkstra Shortest Path
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W06 Prove: Assignment in Canvas for detailed instructions.
*/

use crate::graph::{Graph, INF, GraphError};
use crate::pqueue::PQueue;

/* Find the shortest path from a starting vertex to all
* vertices in a graph using Dijkstra.  Use a PQueue object
* (code already provided for you) in your implementation for the 
* priority queue.  The key for PQueue will be f64 (the distance) and
* the value will be usize (the vertex id from the graph).
*
*  Inputs:
*     g - Graph
*     start - Starting vertex ID
*  Outputs:
*     A Result containing the tuple of the distance and predecessor lists:
*         Ok((Distance List, Predecessor List))
*     NOTE: The Distance List should contain INF as needed.  The Predecessor
*           list should contain Options which contain the vertex id: Some(vertex).
            If there is no predecessor, the error Option should be used: None.
*
*  Errors: If the starting vertex is out of range (invalid vertex id), then the
*          function must return the error Result: Err(GraphError::InvalidVertex)
*/
pub fn shortest_path(g : &Graph, start : usize) -> Result<(Vec<f64>, Vec<Option<usize>>), GraphError> {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test dijkstra_shortest_path
*     Run One Test:  cargo test dijkstra_shortest_path::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_graph1() {
        let mut graph = Graph::new(5);
        let _ = graph.add_edge(0, 1, 6.0);
        let _ = graph.add_edge(0, 3, 4.0);
        let _ = graph.add_edge(1, 2, 3.0);
        let _ = graph.add_edge(1, 3, 2.0);
        let _ = graph.add_edge(2, 4, 4.0);
        let _ = graph.add_edge(3, 1, 1.0);
        let _ = graph.add_edge(3, 2, 9.0);
        let _ = graph.add_edge(3, 4, 3.0);
        let _ = graph.add_edge(4, 2, 5.0);
        let _ = graph.add_edge(4, 0, 7.0);

        let result = shortest_path(&graph, 1);
        assert!(result.is_ok());
        let (dist,pred) = result.unwrap();
        assert_eq!(dist, vec![12.0, 0.0, 3.0, 2.0, 5.0]);
        assert_eq!(pred, vec![Some(4), None, Some(1), Some(1), Some(3)]);
    }

    #[test]
    fn test2_graph2() {
        let mut graph = Graph::new(5);
        let _ = graph.add_edge(0, 1, 4.0);
        let _ = graph.add_edge(0, 3, 7.0);
        let _ = graph.add_edge(1, 0, 3.0);
        let _ = graph.add_edge(1, 2, 6.0);
        let _ = graph.add_edge(2, 1, 6.0);
        let _ = graph.add_edge(2, 4, 3.0);
        let _ = graph.add_edge(3, 2, 1.0);
        let _ = graph.add_edge(4, 0, 4.0);

        let result = shortest_path(&graph, 0);
        assert!(result.is_ok());
        let (dist,pred) = result.unwrap();
        assert_eq!(dist, vec![0.0, 4.0, 8.0, 7.0, 11.0]);
        assert_eq!(pred, vec![None, Some(0), Some(3), Some(0),Some(2)]);
    }
}