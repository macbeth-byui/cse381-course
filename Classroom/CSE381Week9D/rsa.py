# CSE 381 REPL 9C Solution
# RSA

def euclid(a, b):
    if b == 0:
        return (a, 1, 0)
    (gcd, i, j) = euclid(b, a % b)
    return (gcd, j, i - (a//b) * j)

def generatePrivateKey(p, q, e):
    r = (p-1) * (q-1)
    (gcd, i, _) = euclid(e, r)
    # NOTE: Python does the % of negative as we would expect
    # However, to be consistent, we are adding (but not needed)
    if i < 0:
        i += r
    return i % r

# Note that built-in pow function in python will do this for you
def modulo_expo(x, y, n):
    if y == 0:
        return 1
    if y % 2 == 0:
        z = modulo_expo(x, y//2, n)
        return (z * z) % n
    else:
        z = modulo_expo(x, (y-1)//2, n)
        return (z * z * x) % n
    
def encrypt(value, e, n):
    # return (value ** e) % n
    return modulo_expo(value,e,n)

def decrypt(value, d, n):
    # return (value ** d) % n
    return modulo_expo(value,d,n)


