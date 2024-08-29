# CSE 381 REPL 7C Solution
# Arbitrage

from graph import Graph
from graph import INF
import math

# Exchange Rates
#
# 		    USD			EUR			    POUND		YEN	
# USD (1)    			0.93018	        0.79477		139.35200
# EUR (2)   1.07480					    0.85424		149.77100
# POUND (3) 1.25779		1.16985			    		175.29300
# YEN (4)   0.00717		0.00667		    0.00667	

def shortest_path(graph, start_vertex):
    distance = [INF] * graph.size()
    pred = [INF] * graph.size()
    distance[start_vertex] = 0

    for i in range(graph.size()):  # n times (last n is to check negative cycles)
        changesMade = False
        for node in range(0,graph.size()):
            for edge in graph.edges(node):
                if distance[node] + edge.weight < distance[edge.destId]:
                    if i == graph.size()-1:
                        cycle = find_negative_weight_cycle(graph, pred, edge.destId)
                        print(f"Negative Cycle: {cycle}")
                        return (distance,pred)
                    changesMade = True
                    distance[edge.destId] = distance[node] + edge.weight
                    pred[edge.destId] = node
        if not changesMade:
            break
    print("No negative cycle found!")
    return (distance,pred)

def find_negative_weight_cycle(graph, pred, vertex):
    # vertex is either on the cycle or can be reached from the cycle
    visited = [False]*graph.size()
    x = vertex
    # follow the predecessors for vertex until we complete a cycle
    while visited[x] == False:
        visited[x] = True
        x = pred[x]
    # x is a vertex on a negative-weight cycle.  
    v = pred[x]
    cycle = [x]
    # Follow predecessors to capture the full cycle
    while v != x:
        cycle.insert(0,v)
        v = pred[v]
    return cycle
    
    

g = Graph(5);
g.set_label(0, "START")
g.set_label(1, "USD");
g.set_label(2, "EUR");
g.set_label(3, "POUND");
g.set_label(4, "YEN");

# Weight = - log rate (a->b)
g.add_directed_edge(0, 1, 0);
g.add_directed_edge(0, 2, 0);
g.add_directed_edge(0, 3, 0);
g.add_directed_edge(0, 4, 0);
g.add_directed_edge(1, 2, -1 * math.log(0.93018,2))
g.add_directed_edge(1, 3, -1 * math.log(0.79477,2))
#g.add_directed_edge(1, 4, -1 * math.log(139.352,2))
g.add_directed_edge(1, 4, -1 * math.log(140,2))
g.add_directed_edge(2, 1, -1 * math.log(1.0748,2))
g.add_directed_edge(2, 3, -1 * math.log(0.85424,2))
g.add_directed_edge(2, 4, -1 * math.log(149.771,2))
g.add_directed_edge(3, 1, -1 * math.log(1.25779,2))
g.add_directed_edge(3, 2, -1 * math.log(1.16985,2))
g.add_directed_edge(3, 4, -1 * math.log(175.293,2))
#g.add_directed_edge(4, 1, -1 * math.log(0.00717,2))
g.add_directed_edge(4, 1, -1 * math.log(0.00071,2))
g.add_directed_edge(4, 2, -1 * math.log(0.00667,2))
g.add_directed_edge(4, 3, -1 * math.log(0.0057,2))

shortest_path(g, 0);


