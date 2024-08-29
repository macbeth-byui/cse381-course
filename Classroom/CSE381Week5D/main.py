# CSE 381 REPL5D
# Pert Chart

from graph import Graph
from graph import INF
from topo import dag_topological_sort

def dag_shortest_path(graph, start_vertex):
    sorted = dag_topological_sort(graph)
    distance = [INF] * graph.size()
    pred = [INF] * graph.size()

    distance[start_vertex] = 0

    for vertex in sorted:
        if distance[vertex] != INF:
            for edge in graph.edges(vertex):
                if distance[vertex] + edge.weight < distance[edge.destId]:
                    distance[edge.destId] = distance[vertex] + edge.weight
                    pred[edge.destId] = vertex

    return (distance, pred)

g = Graph(34)
g.set_label(0, "Project Start");
g.set_label(1, "Write PSAC");
g.set_label(2, "Write SDP");
g.set_label(3, "Write SVP");
g.set_label(4, "Write SCMP");
g.set_label(5, "Write SQAP");
g.set_label(6, "Review Plans");
g.set_label(7, "Release Plans");
g.set_label(8, "SOI-1");
g.set_label(9, "Release Customer Requirements");
g.set_label(10, "Write SRS");
g.set_label(11, "Review SRS");
g.set_label(12, "Release SRS");
g.set_label(13, "Write SDD");
g.set_label(14, "Review SDD");
g.set_label(15, "Release SDD");
g.set_label(16, "Write Code");
g.set_label(17, "Review Code");
g.set_label(18, "Release Code");
g.set_label(19, "SOI-2");
g.set_label(20, "Write Test Specifications");
g.set_label(21, "Write Test Procedures");
g.set_label(22, "Dry Run Test Procedures");
g.set_label(23, "Review Tests");
g.set_label(24, "Release Tests");
g.set_label(25, "SOI-3");
g.set_label(26, "Write SAS");
g.set_label(27, "Review SAS");
g.set_label(28, "Release SAS");
g.set_label(29, "Formal Test Execution")
g.set_label(30, "SOI-4");
g.set_label(31, "First Article Inspection");
g.set_label(32, "Deliver Software");
g.set_label(33, "Project End");
g.add_directed_edge(0,1,None);
g.add_directed_edge(0,2,None);
g.add_directed_edge(0,3,None);
g.add_directed_edge(0,4,None);
g.add_directed_edge(0,5,None);
g.add_directed_edge(0,9,None);

g.add_directed_edge(1,6,None);

g.add_directed_edge(2,6,None);
g.add_directed_edge(2,10,None);

g.add_directed_edge(3,6,None);

g.add_directed_edge(4,6,None);

g.add_directed_edge(5,6,None);

g.add_directed_edge(6,7,None);

g.add_directed_edge(7,8,None);
g.add_directed_edge(7,11,None);
g.add_directed_edge(7,23,None);

g.add_directed_edge(8,31,None);

g.add_directed_edge(9,10,None);

g.add_directed_edge(10,11,None);

g.add_directed_edge(11,12,None);
g.add_directed_edge(11,13,None);
g.add_directed_edge(11,20,None);

g.add_directed_edge(12,14,None);

g.add_directed_edge(13,14,None);

g.add_directed_edge(14,15,None);
g.add_directed_edge(14,16,None);

g.add_directed_edge(15,17,None);

g.add_directed_edge(16,17,None);
g.add_directed_edge(16,21,None);

g.add_directed_edge(17,18,None);
g.add_directed_edge(17,22,None);

g.add_directed_edge(18,19,None);

g.add_directed_edge(19,31,None);

g.add_directed_edge(20,21,None);

g.add_directed_edge(21,22,None);

g.add_directed_edge(22,23,None);
g.add_directed_edge(22,26,None);

g.add_directed_edge(23,24,None);

g.add_directed_edge(24,25,None);
g.add_directed_edge(24,28,None);

g.add_directed_edge(25,29,None);
g.add_directed_edge(25,31,None);

g.add_directed_edge(26,27,None);

g.add_directed_edge(27,28,None);

g.add_directed_edge(28,30,None);

g.add_directed_edge(29,30,None);

g.add_directed_edge(30,31,None);

g.add_directed_edge(31,32,None);

g.add_directed_edge(32,33,None);

(distance, pred) = dag_shortest_path(g,0)

for i in range(len(distance)):
    print(f"dist of {i} is {distance[i]}")
for i in range(len(pred)):
    print(f"pred of {i} is {pred[i]}")


