/* CSE 381 - Graph
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. 
*
*  Do not modify this file.  You will use this in your code.
*/

use std::slice::Iter;

/* Maintain a directed graph with labels (strings) and weights (f64).  Verticies
 * are represented as indicies in a vertex starting with 0.
 */

pub const INF : f64 = f64::INFINITY;

#[derive(Debug, PartialEq)]
pub enum GraphError {
    InvalidVertex,
    NegativeCycle,
}

pub struct Graph {
    pub vertices : Vec<Vec<Edge>>,
}

pub struct Edge {
    pub dest_id : usize,
    pub weight : f64
}

impl Graph {
    /* Create a new graph using with implied vertex id's of 0, 1, 2, ... size-1.  
     * No edges are created.
     */
    pub fn new(size : usize) -> Self {
        let vertices : Vec<Vec<Edge>> = (0..size).map(|_| Vec::new()).collect();
        Self { vertices }
    }

    /* Add a directed edge between two vertices with a weight.  Err is returned
     * if the verticies are invalid.
     */
    pub fn add_edge(&mut self, src_id : usize, dest_id : usize, weight : f64) -> Result<(), GraphError> {
        if src_id >= self.vertices.len() {
            return Err(GraphError::InvalidVertex);
        }
        if dest_id >= self.vertices.len() {
            return Err(GraphError::InvalidVertex);
        }
        let edge = Edge { dest_id, weight };
        self.vertices[src_id].push(edge);
        Ok(())
    }

    /* Return an iterator cotaining the edges for a vertex.  Err is returned if the 
     * vertex is invalid.
     */
    pub fn edges(&self, vertex_id : usize) -> Result<Iter<'_, Edge>, GraphError> {
        if vertex_id >= self.vertices.len() {
            return Err(GraphError::InvalidVertex);
        }
        Ok(self.vertices[vertex_id].iter())
    }

    /* Return the number of vertices in the graph.
     */
    pub fn size(&self) -> usize {
        self.vertices.len()
    }

}