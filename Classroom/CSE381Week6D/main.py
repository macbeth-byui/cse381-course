# CSE 381 REPL 6D
# Dijkstra Shortest Path

# Source: https://users.cs.utah.edu/~lifeifei/SpatialDataset.htm
# Note that distance is not miles but distance measured on the map

from graph import Graph
from graph import INF
from queue import PriorityQueue2
import time

# Shortest path using a list - O(V^2 + E)
def shortest_path1(graph, start_vertex):
    distance = [INF] * graph.size()
    pred = [INF] * graph.size()
    distance[start_vertex] = 0

    queue = [x for x in range(graph.size())]

    while len(queue) > 0:

        smallest_index = 0
        for index in range(len(queue)):
            if distance[queue[index]] < distance[queue[smallest_index]]:
                smallest_index = index
        smallest_id = queue[smallest_index]
        del queue[smallest_index]

        for edge in graph.edges(smallest_id):
            if distance[smallest_id] + edge.weight < distance[edge.destId]:
                distance[edge.destId] = distance[smallest_id] + edge.weight
                pred[edge.destId] = smallest_id;

    return (distance,pred)

# Shortest Path using a Priority Queue / Heap - O(V log V + E log V)
def shortest_path2(graph, start_vertex):
    distance = [INF] * graph.size()
    pred = [INF] * graph.size()
    distance[start_vertex] = 0

    queue = PriorityQueue2()
    for index in range(graph.size()):
        queue.insert(index, distance[index])

    while queue.size() > 0:

        smallest_id = queue.dequeue()
        for edge in graph.edges(smallest_id):
               if distance[smallest_id] + edge.weight < distance[edge.destId]:
                distance[edge.destId] = distance[smallest_id] + edge.weight
                pred[edge.destId] = smallest_id;
                queue.decrease_key(edge.destId, distance[edge.destId])

    return (distance,pred)

### PART 1 ###

print("Generating Graph")


### PART 2 ###

print("Shortest Path - List Method")


print("Shortest Path - Priority Queue Method")


### PART 3 ###
print("Finding Longest 'Shortest' Distance")


### PART 4 ### 
print("Finding Longest 'Shortest' Path")

