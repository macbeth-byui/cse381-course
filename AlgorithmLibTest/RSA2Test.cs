using System.Numerics;
using AlgorithmLib;
using NUnit.Framework;

namespace AlgorithmLibTest;

[TestFixture]
public class RSA2Test
{
    [Test]
    public void Test1()
    {
        BigInteger p = 87178291199;
        BigInteger q = 22815088913;
        BigInteger e = 65537; // relatively prime to (p-1)*(q-1)
        BigInteger value = 42;
        BigInteger privateKey = RSA2.GeneratePrivateKey(p, q, e);
        Assert.That(privateKey.Equals(BigInteger.Parse("1297782666877314566849")), Is.True);
        BigInteger encrypted = RSA2.Encrypt(value, e, p * q);
        Assert.That(encrypted.Equals(BigInteger.Parse("475967911669796538187")));
        BigInteger decrypted = RSA2.Decrypt(encrypted, privateKey, p * q);
        Assert.That(decrypted.Equals(BigInteger.Parse("42")));
    }

    
}