/* CSE 381 - Bellman Ford
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W07 Prove: Assignment in Canvas for detailed instructions.
*/
use crate::graph::{Graph, INF, GraphError};

/* Find the Shortest Path in a graph using the Bellman Ford Algorithm
*  with the ability to detect a negative cycle.
*
*  Inputs:
*     g - The Graph (using the Graph class provided)
*     start - The vertex ID to calculate shortest path from
*  Outputs:
*     A Result containing the tuple of the distance and predecessor lists:
*         Ok((Distance List, Predecessor List))
*     NOTE: The Distance List should contain INF as needed.  The Predecessor
*           list should contain Options which contain the vertex id: Some(vertex).
            If there is no predecessor, the error Option should be used: None.
*
*  Errors: If a negative cycle exists, then the function must return
*          the error Result: Err(GraphError::NegativeCycle)
*
*          If the starting vertex is out of range (invalid vertex id), then the
*          function must return the error Result: Err(GraphError::InvalidVertex)
*/
pub fn shortest_path(g : &Graph, start : usize) -> Result<(Vec<f64>, Vec<Option<usize>>), GraphError> {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test bellman_ford_shortest_path
*     Run One Test:  cargo test bellman_ford_shortest_path::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_no_negative_cycle() {
        let mut graph = Graph::new(5);
        let _ = graph.add_edge(0, 1, 6.0);
        let _ = graph.add_edge(0, 3, 7.0);
        let _ = graph.add_edge(1, 2, 5.0);
        let _ = graph.add_edge(1, 3, 8.0);
        let _ = graph.add_edge(1, 4, -4.0);
        let _ = graph.add_edge(2, 1, -2.0);
        let _ = graph.add_edge(3, 2, -3.0);
        let _ = graph.add_edge(3, 4, 9.0);
        let _ = graph.add_edge(4, 0, 2.0);
        let _ = graph.add_edge(4, 2, 7.0);

        let result = shortest_path(&graph, 0);
        assert!(result.is_ok());
        let (dist,pred) = result.unwrap();
        assert_eq!(dist, vec![0.0, 2.0, 4.0, 7.0, -2.0]);
        assert_eq!(pred, vec![None, Some(2), Some(3), Some(0), Some(1)]);
    }

    #[test]
    fn test2_negative_cycle() {
        let mut graph = Graph::new(5);
        let _ = graph.add_edge(0, 1, 6.0);
        let _ = graph.add_edge(0, 3, 7.0);
        let _ = graph.add_edge(1, 2, 5.0);
        let _ = graph.add_edge(1, 3, -1.0);
        let _ = graph.add_edge(1, 4, -4.0);
        let _ = graph.add_edge(2, 1, -2.0);
        let _ = graph.add_edge(3, 2, -3.0);
        let _ = graph.add_edge(3, 4, 9.0);
        let _ = graph.add_edge(4, 0, 2.0);
        let _ = graph.add_edge(4, 2, 7.0);

        let result = shortest_path(&graph, 0);
        assert_eq!(result, Err(GraphError::NegativeCycle));
    }
}