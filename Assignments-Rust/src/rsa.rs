/* CSE 381 - RSA 
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W09 Prove: Assignment in Canvas for detailed instructions.
*/

use num_bigint::BigInt;
use num_traits::{Zero, One};

/* Recursively use Euclid to find the Greatest Common Divisor between
* two numbers as well as the linear combination form.
*
*  Inputs:
*     a - First number
*     b - Second number
*  Outputs:
*     (gcd, i, j) where gcd = i*a + j*b
*/
fn euclid(a : &BigInt, b : &BigInt) -> (BigInt, BigInt, BigInt) {
    todo!()
}

/* Recursively calculates x^y mod n
*
*  Inputs:
*     x - base
*     y - exponent
*     n - modulo
*  Outputs:
*     Result of x^y mod n
*/
fn mod_expo(x : &BigInt, y : &BigInt, n : &BigInt) -> BigInt {
    todo!()
}

/* Generate the RSA private key given the two prime numbers p and q and
*  the public key e which was selected to be co-prime with
*  phi = (p-1) * (q-1).
* 
*  Inputs:
*     p - First prime
*     q - Second prime
*     e - Public Key 
*  Outputs:
*     Private Key - Must be positive
*/
pub fn generate_private_key(p : &BigInt, q : &BigInt, e : &BigInt) -> BigInt {
    todo!()
}

/* Encrypt a value using the public keys e and n
*
*  Inputs:
*     value - Value to encrypt
*     e - Public Key whose value was co-prime with phi
*     n - Public Key whose Value is equal to p*q
*  Outputs:
*     encrypted value
*/
pub fn encrypt(value : &BigInt, e : &BigInt, n : &BigInt) -> BigInt {
    todo!()
}

/* Decrypt a value using the public key n and private key d
*
*  Inputs:
*     value - Value to decrypt
*     d - Private Key whose value was the multiplicative inverse of e mod phi
*     n - Public Key whose Value is equal to p*q
*  Outputs:
*     encrypted value
*/
pub fn decrypt(value : &BigInt, d : &BigInt, n : &BigInt) -> BigInt {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test rsa
*     Run One Test:  cargo test rsa::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_euclid_small() {
        let a = BigInt::from(8);
        let b = BigInt::from(12);

        let (gcd, i, j) = euclid(&a, &b);
        assert_eq!(gcd, BigInt::from(4));
        assert_eq!(i, BigInt::from(-1));
        assert_eq!(j, BigInt::from(1));
    }

    #[test]
    fn test2_euclid_coprime_small() {
        let a = BigInt::from(5);
        let b = BigInt::from(72);

        let (gcd, i, j) = euclid(&a, &b);
        assert_eq!(gcd, BigInt::from(1));
        assert_eq!(i, BigInt::from(29));
        assert_eq!(j, BigInt::from(-2)); 
    }

    #[test]
    fn test3_euclid_coprime_big() {
        let a = BigInt::from(65537);
        let b = ("87178291199".parse::<BigInt>().unwrap() - 1) * 
                        ("22815088913".parse::<BigInt>().unwrap() - 1);
        
        let (gcd, i, j) = euclid(&a, &b);
        assert_eq!(gcd, BigInt::from(1));
        assert_eq!(i, "-691197798001282429727".parse::<BigInt>().unwrap());
        assert_eq!(j, BigInt::from(22775)); 
    }

    #[test]
    fn test4_mod_expo() {
        let a = BigInt::from(3);
        let b = BigInt::from(50);
        let n = BigInt::from(5);
        let result = mod_expo(&a, &b, &n);
        assert_eq!(result, BigInt::from(4));
    }

    #[test]
    fn test5_generate_private_key() {
        let p = "87178291199".parse::<BigInt>().unwrap();
        let q = "22815088913".parse::<BigInt>().unwrap();
        let e = "65537".parse::<BigInt>().unwrap();
        let d = generate_private_key(&p, &q, &e);
        assert_eq!(d, "1297782666877314566849".parse::<BigInt>().unwrap());
    }

    #[test]
    fn test7_encrypt() {
        let p = "87178291199".parse::<BigInt>().unwrap();
        let q = "22815088913".parse::<BigInt>().unwrap();
        let e = "65537".parse::<BigInt>().unwrap();
        let n = p * q;
        let value = BigInt::from(42);
        let encrypted = encrypt(&value, &e, &n);
        assert_eq!(encrypted, "475967911669796538187".parse::<BigInt>().unwrap());
    }

    #[test]
    fn test8_decrypt() {
        let p = "87178291199".parse::<BigInt>().unwrap();
        let q = "22815088913".parse::<BigInt>().unwrap();
        let e = "65537".parse::<BigInt>().unwrap();
        let d = generate_private_key(&p, &q, &e);
        let n = p * q;
        let value = BigInt::from(42);
        let encrypted = encrypt(&value, &e, &n);
        let decrypted = decrypt(&encrypted, &d, &n);
        assert_eq!(decrypted, value);
    }


}
