/* CSE 381 - Priority Queue
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site.
*
*  Do not modify this file.  You will use this in your code.
*/
use std::collections::HashMap;
use std::hash::Hash;

/* Implement a priority queue using a binary heap.  Each node in the heap will contain a value (which is a generic type V
*  that implements Eq and Hash and Clone) and a priority key (which is generic type K that implements PartialOrd)  The heap is implemented 
*  as an array to provide faster access to the end of the heap.  A lookup table is implemented to provide quick access to a 
*  node in the heap to allow for decreasing the the priority key.  The node with the lowest priority key will be the root of the heap.
*/
struct Node<K : PartialOrd, V : Eq + Hash + Clone> {
    priority_key : K,
    value : V,
}

pub struct PQueue<K : PartialOrd, V : Eq + Hash + Clone> {
    heap : Vec<Node<K,V>>,
    lookup : HashMap<V, usize>,
}

impl<K : PartialOrd, V : Eq + Hash + Clone> Default for PQueue<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K : PartialOrd, V : Eq + Hash + Clone> PQueue<K, V> {
    /* Create an empty heap.
     */
    pub fn new() -> Self {
        let heap = Vec::<Node<K,V>>::new();
        let lookup = HashMap::<V, usize>::new();
        Self {heap, lookup}
    }

    /* Get the parent node based on index.  Returns None
     * if the index is not valid or there is no parent.
     */
    fn get_parent(&self, index : usize) -> Option<usize> {
        if index == 0 || index >= self.size() {
            return None;
        }
        Some((index - 1) / 2)
    }

    /* Get the left node based on index.  Returns None
     * if the index is invalid or there is no left child
     */
    fn get_left(&self, index : usize) -> Option<usize> {
        if index >= self.size() {
            return None;
        }        
        let left = (index * 2) + 1;
        if left >= self.size() { None } else { Some(left) }
    }

    /* Get the right node based on index.  Returns None
     * if the index is invalid or there is no right child.
     */
    fn get_right(&self, index : usize) -> Option<usize> {
        if index >= self.size() {
            return None;
        }
        let right = (index * 2) + 2;
        if right >= self.size() { None } else { Some(right) }
    }

    /* Bubble up a node so that the priority_key of a parent is always
     * less than or equal to the priority_key of the children.
     */
    fn bubble_up(&mut self, mut curr : usize) {
        loop {
            match self.get_parent(curr) {
                Some(parent) => {
                    // Properly placed already
                    if self.heap[parent].priority_key <= self.heap[curr].priority_key {
                        return;
                    }
                    // Move up
                    self.bubble_apply(&mut curr, parent);
                }
                None => return  // No Parent (must be the root or empty)
            }
        }
    }

    /* Bubble down a node so that the priority_key of a parent is always 
     * less than or equal to the priority_key of the children.
     */
    fn bubble_down(&mut self, mut curr : usize) {
        loop {
            match (self.get_left(curr), self.get_right(curr)) {
                (Some(left), Some(right)) => {
                    // Properly placed already
                    if self.heap[curr].priority_key <= self.heap[left].priority_key &&
                       self.heap[curr].priority_key <= self.heap[right].priority_key {
                        return;
                    }
                    // Move down to the left
                    if self.heap[left].priority_key <= self.heap[right].priority_key {
                        self.bubble_apply(&mut curr, left);
                    } 
                    // Move down to the right
                    else {
                        self.bubble_apply(&mut curr, right);
                    }
                }
                (Some(left), None) => {
                    // Properly placed already
                    if self.heap[curr].priority_key <= self.heap[left].priority_key {
                        return;
                    }
                    // Move down to the left
                    self.bubble_apply(&mut curr, left)
                }
                (None, Some(right)) => {
                    // Properly placed already
                    if self.heap[curr].priority_key <= self.heap[right].priority_key {
                        return;
                    }
                    // Move down to the right
                    self.bubble_apply(&mut curr, right)
                }
                (None, None) => return // No children (must be leaf)
            }
        }

    }

    /* Utility to swap, update lookup, and move current for the bubble functions
     */
    fn bubble_apply(&mut self, curr : &mut usize, target : usize) {
        // Swap, update lookup, and move 
        self.heap.swap(target, *curr);
        self.lookup.insert(self.heap[target].value.clone(), target);
        self.lookup.insert(self.heap[*curr].value.clone(), *curr);
        *curr = target;
    }

    /* Reduce the priority_key and move the node up.  Returns None if priority_key
     * did not go down or if the value is invalid.
     */
    pub fn decrease_priority(&mut self, value : V, priority_key : K) -> Option<()> {
        if let Some(curr) = self.lookup.get(&value) {
            if priority_key >= self.heap[*curr].priority_key {
                return None;
            }
            // Change the distance and move up
            self.heap[*curr].priority_key = priority_key;
            self.bubble_up(*curr);
            return Some(());
        }       
        None
    }

    /* Add a new node to the heap.
     */
    pub fn enqueue(&mut self, value : V, priority_key : K) {
        // Add to the end and move up as needed
        self.heap.push(Node {value : value.clone(), priority_key});
        self.lookup.insert(value, self.heap.len() -1);
        self.bubble_up(self.heap.len() - 1);
    }

    /* Remove the root node.  Return None if heap is already empty.
     */
    pub fn dequeue(&mut self) -> Option<V> {
        if self.heap.is_empty() {
            return None;
        }
        // Swap last and first
        let value = self.heap[0].value.clone();
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        self.lookup.insert(self.heap[0].value.clone(), 0);

        // Remove last
        self.heap.pop();
        self.lookup.remove(&value);

        // Move first down as needed
        self.bubble_down(0);
        Some(value)
    }

    /* Return size of the heap
     */
    pub fn size(&self) -> usize {
        self.heap.len()
    }


}