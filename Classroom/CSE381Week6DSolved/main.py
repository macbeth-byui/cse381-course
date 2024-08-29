# CSE 381 REPL 6D Solution
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

print("Generating Graph")

with open("ca_roads_verticies.txt") as fd:
    verticies = fd.readlines()

g = Graph(len(verticies))

with open("ca_roads_edges.txt") as fd:
    for line in fd:
        fields = line.strip().split(" ")
        src_id = int(fields[1])
        dst_id = int(fields[2])
        distance = float(fields[3])
        g.add_undirected_edge(src_id, dst_id, distance)

print(f"Graph Generated: V={g.size()} E={g.get_size_edges()}")

start = time.perf_counter()
dist, pred = shortest_path1(g, 0)
stop = time.perf_counter()
print(f"shortest_path1 = {stop-start}")

start = time.perf_counter()
dist, pred = shortest_path2(g, 0)
stop = time.perf_counter()
print(f"shortest_path2 = {stop-start}")

# Maximum distance
max_dist = 0
max_index = -1
for index in range(len(dist)):
    if dist[index] > max_dist:
        max_dist =  dist[index]
        max_index = index
print(f"max_index = {max_index}, max_dist = {max_dist}")

# Path from max distance to starting point
start = 0
curr = max_index
while True:
    print(f"ID: {curr} Total Distance: {dist[curr]}")
    if curr == start:
        break
    curr = pred[curr]

