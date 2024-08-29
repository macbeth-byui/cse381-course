# CSE 381 Workshop 7
# Do not modify this file

class PriorityQueue:

    class PqNode:

        def __init__(self, value, priority):
            self.priority = priority
            self.value = value

    def __init__(self):
        self._heap = []
        self._lookup = {}

    def parent(self,index):
        return (index - 1) // 2

    def is_leaf(self,index):
        return index >= (len(self._heap) // 2)

    def left(self,index):
        return 2 * index + 1

    def right(self,index):
        return 2 * index + 2

    def bubble_up(self, currNode):
        while currNode > 0:
            parentNode = self.parent(currNode)
            if self._heap[parentNode].priority > self._heap[currNode].priority:
                self._heap[parentNode], self._heap[currNode] = self._heap[currNode], self._heap[parentNode]
                self._lookup[self._heap[currNode].value] = currNode
                self._lookup[self._heap[parentNode].value] = parentNode
            currNode = parentNode

    def bubble_down(self, currNode):
        while not self.is_leaf(currNode):
            leftNode = self.left(currNode)
            rightNode = self.right(currNode)
            if (rightNode >= len(self._heap)) or (self._heap[leftNode].priority <= self._heap[rightNode].priority):
                if self._heap[currNode].priority > self._heap[leftNode].priority:
                    self._heap[currNode], self._heap[leftNode] = self._heap[leftNode], self._heap[currNode]
                    self._lookup[self._heap[currNode].value] = currNode
                    self._lookup[self._heap[leftNode].value] = leftNode
                currNode = leftNode
            else:
                if self._heap[currNode].priority > self._heap[rightNode].priority:
                    self._heap[currNode], self._heap[rightNode] = self._heap[rightNode], self._heap[currNode]
                    self._lookup[self._heap[currNode].value] = currNode
                    self._lookup[self._heap[rightNode].value] = rightNode
                currNode = rightNode


    def decrease_key(self, value, priority):
        if value in self._lookup:
            curr = self._lookup[value]
            self._heap[curr].priority = priority
            self.bubble_up(curr)

    def insert(self, value, priority):
        newNode = PriorityQueue.PqNode(value, priority)
        self._heap.append(newNode)
        curr = len(self._heap) - 1
        self._lookup[value] = curr;
        self.bubble_up(curr)

    def dequeue(self):
        if len(self._heap) == 0:
            raise Exception("Priority Queue was empty")

        result = self._heap[0]
        last = self._heap[len(self._heap)-1]
        self._heap[0] = last
        self._lookup[self._heap[0].value] = 0
        self._heap.pop()
        self._lookup.pop(result.value)
        self.bubble_down(0)

        return result.value

    def size(self):
        return len(self._heap)

    def print_heap(self):
        for pair in self._heap:
            print(f"{pair.value} (p{pair.priority}), ", end="")
        print()

# heap = PriorityQueue2()
# heap.insert("A",7);
# heap.print_heap();
# heap.insert("B",6);
# heap.print_heap();
# heap.insert("C",4);
# heap.print_heap();
# heap.insert("D",8);
# heap.print_heap();
# heap.insert("E",3);
# heap.print_heap();
# heap.insert("F",2);
# heap.print_heap();
# print("-----");
# heap.decrease_key("D",0);
# heap.print_heap();
# heap.decrease_key("C",1);
# heap.print_heap();
# print("-----");
# heap.dequeue();
# heap.print_heap();
# heap.dequeue();
# heap.print_heap();
# heap.dequeue();
# heap.print_heap();
# heap.dequeue();
# heap.print_heap();
# heap.dequeue();
# heap.print_heap();
# heap.dequeue();
# heap.print_heap();