import timeit


class Trie:

    class Node:

        def __init__(self, letter):
            self.letter = letter  # Value 
            self.children = {}    # How many other Values follow it
            self.match = False    # Does this terminate a word

    def __init__(self):
        self.root = Trie.Node("") # Start out empty

    # Wrapper to start at root
    def add(self, word):
        self.add_(word, self.root)

    def add_(self, word, node):
        # Out of letters then this node is terminating
        if len(word) == 0:
            node.match = True
        # If we have seen this letter before, then follow it
        elif word[0] in node.children:
            self.add_(word[1:], node.children[word[0]])
        # If we have a new letter, create it, add it to the children, and follow it
        else:
            new_node = Trie.Node(word[0])
            node.children[word[0]] = new_node
            self.add_(word[1:], new_node)

    # Wrapper to start at root and convert to boolean
    def find(self, word):
        node = self.find_(word, self.root)
        if node is None:
            return False
        return node.match  # If we found it but it wasn't terminating, then word doesn't exist

    def find_(self, word, node):
        # If we are out of letters then we found the node (might not be terminating though)
        if len(word) == 0:
            return node
        # Next letter is in the children so follow it
        elif word[0] in node.children:
            return self.find_(word[1:], node.children[word[0]])
        # Next letter not there, nothing found.
        else:
            return None

    # Wrapper to find the node and then follow it
    def possible(self, prefix):
        starting_node = self.find_(prefix, self.root)
        if starting_node is None:
            return []

        # Find all words starting from this node
        results = []
        self.possible_(prefix, starting_node, results)
        return results

    def possible_(self, prefix, node, results):
        # This is a terminating words so include it
        if node.match:
            results.append(prefix)
        # Follow all the children looking for more terminating words
        for (letter,child) in node.children.items():
            self.possible_(prefix+letter, child, results)

# Find all words with the same prefix
def list_possible(prefix, words):
    results = []
    for word in words:
        if word.startswith(prefix):
            results.append(word)
    return results


# Read the words into a list and clean them up
print("Reading words from words.txt")

with open("words.txt") as f:
    words = f.readlines()
for i in range(len(words)):
    words[i] = words[i].strip().lower()

print(f"Total Words: {len(words)}")
print()

# Load the words into the Trie
print("Loading words into the Trie")

trie = Trie()
for word in words:
    trie.add(word)    


# Shows results and timing for user entered words

while True:
    word = input("Enter word: ")
    print()

    print(f"Find in List: {word in words}")
    print(f"Find in Trie: {trie.find(word)}")
    print()

    time1 = timeit.timeit(stmt = "word in words", globals=globals(), number=10) * 1000 / 10
    time2 = timeit.timeit(stmt = "trie.find(word)", globals=globals(), number=10) * 1000 / 10
    print(f"Find List = {time1:.4f} ms Trie = {time2:.4f} ms")
    print()

    print(f"Possible in List (First 10): {list_possible(word,words)[:10]}")
    print(f"Possible in Trie (First 10): {trie.possible(word)[:10]}")
    print()

    time1 = timeit.timeit(stmt = "list_possible(word,words)", globals=globals(), number=10) * 1000 / 10
    time2 = timeit.timeit(stmt = "trie.possible(word)", globals=globals(), number=10) * 1000 / 10
    print(f"Possible List = {time1:.4f} ms Trie = {time2:.4f} ms")
    print()


