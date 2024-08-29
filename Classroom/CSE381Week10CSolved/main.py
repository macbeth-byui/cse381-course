# CSE 381 REPL 10C Solution
# Huffman Trees

import functools
import pickle
from huffman import profile, build_tree, create_encoding_map
from huffman import encode, decode

def convert_to_bit_stream(text):
    # Convert the string of 1's and 0's to a bytearray
    start = 0
    result = bytearray()
    while start < len(text):
        # Take blocks of 8 bits at a time and interpret them in base 2
        # This integer is then converted to a byte and stored in the bytearray.
        num = int(text[start:start+8], 2)
        result.append(num)
        start += 8
    return result

def convert_from_bit_stream(bytes):
    # Convert the bytearray to a string of 1's and 0's
    result = ""
    for index in range(len(bytes)):
        # For each byte, convert to a string of 1's and 0's
        # Remove the '0b' from the front. We need to have the
        # leading 0's added back.  However, for the last byte,
        # we shouldn't add additional 0's.
        binary = bin(bytes[index])[2:]
        if index < len(bytes)-1:
            binary = binary.zfill(8)
        result += binary
    return result

print("Reading file...")
fd = open("book-of-mormon.json","r")
lines = fd.readlines()
fd.close()
text = functools.reduce(lambda a, b : a+b, lines)

print("Compressing file...")
profile = profile(text);
tree = build_tree(profile);
encoding_map = create_encoding_map(tree);
encoded = encode(text, encoding_map);
bit_stream = convert_to_bit_stream(encoded)

print("Writing file...")
fd = open("book-of-mormon.zip", "wb")
pickle.dump(profile, fd)
fd.write(bit_stream)
fd.close()

print("Reading Zip file...")
fd = open("book-of-mormon.zip", "rb")
profile = pickle.load(fd)
tree = build_tree(profile)
bit_stream = fd.read()
fd.close()

print("Extracting Zip file...")
encoded = convert_from_bit_stream(bit_stream)
text = decode(encoded, tree)
fd = open("book-of-mormon2.json", "w", newline='')
fd.write(text)
fd.close()
