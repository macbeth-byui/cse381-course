// CSE 381 REPL 5A
// Graph API

public class Edge
{
    public int DestId { get; set; }
    public int Weight { get; set; }
}

public class Graph
{
    private List<List<Edge>> _graph; 
    private List<string> _labels;

    public Graph(int size)
    {
        _graph = Enumerable.Range(0, size).Select(_ => new List<Edge>()).ToList();
        _labels = Enumerable.Range(0, size).Select(_ => "").ToList();
    }

    public void SetLabel(int id, string label)
    {
        _labels[id] = label;
    }

    public void AddDirectedEdge(int srcId, int destId, int weight=0)
    {

    }

    public void AddUndirectedEdge(int srcId, int destId, int weight=0)
    {

    }

    public List<Edge> Edges(int id)
    {
        return _graph[id];
    }

    public int Size()
    {
        return _graph.Count;
    }

    public string GetLabel(int id)
    {
        return _labels[id];
    }
}

class Program
{
    public static void Main(string[] args) 
    {
        var g = new Graph(4);
        g.SetLabel(0,"A");
        g.SetLabel(1,"B");
        g.SetLabel(2,"C");
        g.SetLabel(3,"D");

        
        for (var vertex = 0; vertex<g.Size(); vertex++) 
        {
            Console.Write($"{g.GetLabel(vertex)}: ");
            foreach (var edge in g.Edges(vertex)) 
            {
                Console.Write($" ({g.GetLabel(edge.DestId)}, {edge.Weight})");    
            }
            Console.WriteLine();
        }
    }
}