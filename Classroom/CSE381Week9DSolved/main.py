# CSE 381 REPL 9D Solution
# Encrypt Files

from rsa import encrypt, decrypt
import random

n = 1988980464988590376687
public_key = 65537
private_key = 1297782666877314566849

fd = open("poem.txt","r")
lines = fd.readlines()
fd.close()

fd = open("poem.enc","w")
for line in lines:
    for letter in line:
        salt = random.randint(0,999)
        salted_value = ord(letter)*1000 + salt
        enc = encrypt(salted_value,public_key,n)
        fd.write(str(enc)+"\n")
fd.close()

fd = open("poem.enc","r")
lines = fd.readlines()
fd.close()
for line in lines:
    value = decrypt(int(line), private_key, n)
    saltless_value = chr(value // 1000)
    print(saltless_value,end="")