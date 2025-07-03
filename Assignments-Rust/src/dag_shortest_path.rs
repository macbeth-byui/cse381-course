/* CSE 381 - DAG Shortest Path
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W05 Prove: Assignment in Canvas for detailed instructions.
*/

use crate::graph::{Graph, GraphError, INF};

/* Topologically sort all vertices in a graph and return the sorted
*  list of vertex ID's.  Use a Stack object (available in C#).
*
*  Inputs:
*     g - Graph
*  Outputs:
*     Return a sorted list of vertex ID's
*/
fn sort(g : &Graph) -> Vec<usize> {
    todo!()
}

/* Find the shortest path from a starting vertex to all
*  vertices in a DAG.  This function will need to
*  all the Sort function to obtain the topologically
*  sorted list of vertices from the graph.
*
*  Inputs:
*     g - Directed Acyclic Graph
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
*     Run All Tests: cargo test dag_shortest_path
*     Run One Test:  cargo test dag_shortest_path::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_topo_sort() {
        let mut graph = Graph::new(9);
        let _ = graph.add_edge(0, 3, 3.0);
        let _ = graph.add_edge(1, 3, 2.0);
        let _ = graph.add_edge(2, 5, 5.0);
        let _ = graph.add_edge(3, 6, 9.0);
        let _ = graph.add_edge(5, 6, 3.0);
        let _ = graph.add_edge(4, 8, 7.0);
        let _ = graph.add_edge(6, 8, 7.0);
        let _ = graph.add_edge(7, 8, 5.0);

        let result = sort(&graph);
        assert_eq!(result, vec![7, 4, 2, 5, 1, 0, 3, 6, 8]);
    }

    #[test]
    fn test2_dag1() {
        let mut graph = Graph::new(9);
        let _ = graph.add_edge(0, 3, 3.0);
        let _ = graph.add_edge(1, 3, 2.0);
        let _ = graph.add_edge(2, 5, 5.0);
        let _ = graph.add_edge(3, 6, 9.0);
        let _ = graph.add_edge(5, 6, 3.0);
        let _ = graph.add_edge(4, 8, 7.0);
        let _ = graph.add_edge(6, 8, 7.0);
        let _ = graph.add_edge(7, 8, 5.0);

        let result = shortest_path(&graph, 2);
        assert!(result.is_ok());
        let (dist, pred) = result.unwrap();
        assert_eq!(dist, vec![INF, INF, 0.0, INF, INF, 5.0, 8.0, INF, 15.0]);
        assert_eq!(pred, vec![None, None, None, None, None, Some(2), Some(5), None, Some(6)]);
    }

    #[test]
    fn test3_dag2() {
        let mut graph = Graph::new(6);
        let _ = graph.add_edge(0, 1, 5.0);
        let _ = graph.add_edge(0, 2, 3.0);
        let _ = graph.add_edge(1, 2, 2.0);
        let _ = graph.add_edge(1, 3, 6.0);
        let _ = graph.add_edge(2, 3, 7.0);
        let _ = graph.add_edge(2, 4, 4.0);
        let _ = graph.add_edge(2, 5, 2.0);
        let _ = graph.add_edge(3, 4, -1.0);
        let _ = graph.add_edge(3, 5, 1.0);
        let _ = graph.add_edge(4, 5, -2.0);

        let result = shortest_path(&graph, 1);
        assert!(result.is_ok());
        let (dist, pred) = result.unwrap();
        assert_eq!(dist, vec![INF, 0.0, 2.0, 6.0, 5.0, 3.0]);
        assert_eq!(pred, vec![None, None, Some(1), Some(1), Some(3), Some(4)]);
    }
}