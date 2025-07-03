/* CSE 381 - Bellman Ford
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W07 Prove: Assignment in Canvas for detailed instructions.
*/

namespace AlgorithmLib;

public static class BellmanFordShortestPath
{
    /* Find the Shortest Path in a graph using the Bellman Ford Algorithm
    *  with the ability to detect a negative cycle.
    *
    *  Inputs:
    *     g - The Graph (using the Graph class provided)
    *     startVertex - The vertex ID to calculate shortest path from
    *  Outputs:
    *     (Distance List, Predecessor List)
    *     NOTE: The above two output lists should contain Graph.INF as needed
    *
    *  Note: If a negative cycle exists, then the function must return
    *  a tuple of two empty lists. 
    */
    public static (List<int>, List<int>) ShortestPath(Graph g, int startVertex)
    {
        return (new List<int>(), new List<int>());
    } 
}