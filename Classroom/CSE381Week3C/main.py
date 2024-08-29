# CSE 381 REPL3C 
# Sorting Objects with lambda

def Sort(data):
    _Sort(data, 0, len(data)-1)

def _Sort(data, first, last):
    if first >= last:  # Item of size 1 is sorted
        return
    mid = (first+last) // 2
    _Sort(data, first, mid)
    _Sort(data, mid+1, last)
    Merge(data, first, mid, last)

def Merge(data, first, mid, last):
    sa1 = data[first:mid+1]
    sa2 = data[mid+1:last+1]
    sa1Index = 0
    sa2Index = 0
    for mIndex in range(first,last+1):
        if sa1Index >= len(sa1):
            data[mIndex] = sa2[sa2Index]
            sa2Index += 1
        elif sa2Index >= len(sa2):
            data[mIndex] = sa1[sa1Index]
            sa1Index += 1
        elif sa1[sa1Index] < sa2[sa2Index]:
            data[mIndex] = sa1[sa1Index]
            sa1Index += 1
        else:
            data[mIndex] = sa2[sa2Index]
            sa2Index += 1

class Product:

    def __init__(self, name, price, qty):
        self.name = name
        self.price = price
        self.qty = qty

    def __str__(self):
        return f"{self.name} - ${self.price} ({self.qty})"

data = [Product("Bike",60.95,10),
        Product("Fishing Pole", 30.99, 30),
        Product("Train",45.55, 12),
        Product("Cards", 5.97, 40),
        Product("Stickers", 0.99, 1000),
        Product("Pillow", 15.97, 20),
        Product("Ball", 2.39, 25)]

# Sort by name
# Sort(data, key = lambda x : ?????)


# Sort by price


# Sort by qty


# Sort by value of stock (qty * value)


# Sort by length of name


for item in data:
    print(item)