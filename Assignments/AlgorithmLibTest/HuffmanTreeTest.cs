/*  CSE 381 - Huffman Tree Test
 *  (c) BYU-Idaho - It is an honor code violation to post this
 *  file completed in a public file sharing site.
 *
 *  Instructions: Do not modify this file.  Use these test to verify
 *  that your code is working properly.
*/
#pragma warning disable CS8602   // Dereference of a possibly null reference.

using AlgorithmLib;
using NUnit.Framework;

namespace AlgorithmLibTest;

[TestFixture]
public class HuffmanTreeTest
{
    
    [Test]
    public void Test1_Profile()
    {
        var text = "the rain in spain stays mainly in the plain";
        var profile = HuffmanTree.Profile(text);
        Assert.That(profile[' '], Is.EqualTo(8));
        Assert.That(profile['a'], Is.EqualTo(5));
        Assert.That(profile['e'], Is.EqualTo(2));
        Assert.That(profile['h'], Is.EqualTo(2));
        Assert.That(profile['i'], Is.EqualTo(6));
        Assert.That(profile['l'], Is.EqualTo(2));
        Assert.That(profile['m'], Is.EqualTo(1));
        Assert.That(profile['n'], Is.EqualTo(6));
        Assert.That(profile['p'], Is.EqualTo(2));
        Assert.That(profile['r'], Is.EqualTo(1));
        Assert.That(profile['s'], Is.EqualTo(3));
        Assert.That(profile['t'], Is.EqualTo(3));
        Assert.That(profile['y'], Is.EqualTo(2));
        Assert.That(profile.Count, Is.EqualTo(13));
        Assert.Pass();
    }
    
    [Test]
    public void Test2_BuildTree()
    {
        var text = "the rain in spain stays mainly in the plain";
        var profile = HuffmanTree.Profile(text);
        var tree = HuffmanTree.BuildTree(profile);

        void Traverse(HuffmanTree.Node? node)
        {
            if (node.Left == null && node.Right == null)
            {
                Assert.That(node.Count, Is.EqualTo(profile[node.Letter]));
                return;
            }
            Assert.That(node.Left, Is.Not.Null);
            Assert.That(node.Right, Is.Not.Null);
            Assert.That(node.Count, Is.EqualTo(node.Left.Count + node.Right.Count));
            Traverse(node.Left);
            Traverse(node.Right);
        }

        Traverse(tree);
        
        Assert.Pass();
    }
    
    [Test]
    public void Test3_CreateEncodingMap()
    {
        var tree = new HuffmanTree.Node {
            Count = 43,
            Left = new HuffmanTree.Node {
                Count = 17,
                Left = new HuffmanTree.Node {
                    Count = 8,
                    Left = new HuffmanTree.Node {
                        Count = 4,
                        Left = new HuffmanTree.Node { Count = 2, Letter = 'l' },
                        Right = new HuffmanTree.Node { Count = 2, Letter = 'e' }
                    },
                    Right = new HuffmanTree.Node {
                        Count = 4,
                        Left = new HuffmanTree.Node { Count = 2, Letter = 'h' },
                        Right = new HuffmanTree.Node {
                            Count = 2,
                            Left = new HuffmanTree.Node { Count = 1, Letter = 'r' },
                            Right = new HuffmanTree.Node { Count = 1, Letter = 'm' }
                        }
                    }
                },
                Right = new HuffmanTree.Node {
                    Count = 9,
                    Left = new HuffmanTree.Node {
                        Count = 4,
                        Left = new HuffmanTree.Node { Count = 2, Letter = 'p' },
                        Right = new HuffmanTree.Node { Count = 2, Letter = 'y' }
                    },
                    Right = new HuffmanTree.Node { Count = 5, Letter = 'a' }
                }
            },
            Right = new HuffmanTree.Node {
                Count = 26,
                Left = new HuffmanTree.Node {
                    Count = 12,
                    Left = new HuffmanTree.Node {
                        Count = 6,
                        Left = new HuffmanTree.Node { Count = 3, Letter = 's' },
                        Right = new HuffmanTree.Node { Count = 3, Letter = 't' }
                    },
                    Right = new HuffmanTree.Node { Count = 6, Letter = 'n' }
                },
                Right = new HuffmanTree.Node {
                    Count = 14,
                    Left = new HuffmanTree.Node { Count = 6, Letter = 'i' },
                    Right = new HuffmanTree.Node { Count = 8, Letter = ' ' }
                }
            }
        };

        var map = HuffmanTree.CreateEncodingMap(tree);
        Assert.That(map['l'], Is.EqualTo("0000"));
        Assert.That(map['e'], Is.EqualTo("0001"));
        Assert.That(map['h'], Is.EqualTo("0010"));
        Assert.That(map['r'], Is.EqualTo("00110"));
        Assert.That(map['m'], Is.EqualTo("00111"));
        Assert.That(map['p'], Is.EqualTo("0100"));
        Assert.That(map['y'], Is.EqualTo("0101"));
        Assert.That(map['a'], Is.EqualTo("011"));
        Assert.That(map['s'], Is.EqualTo("1000"));
        Assert.That(map['t'], Is.EqualTo("1001"));
        Assert.That(map['n'], Is.EqualTo("101"));
        Assert.That(map['i'], Is.EqualTo("110"));
        Assert.That(map[' '], Is.EqualTo("111"));
        Assert.That(map.Count, Is.EqualTo(13));
        Assert.Pass();

    }
    
    [Test]
    public void Test4_Encode()
    {
        var text = "the rain in spain stays mainly in the plain";
        var map = new Dictionary<char, string>
        {
            ['l'] = "11101",
            ['e'] = "11111",
            ['h'] = "11110",
            ['r'] = "01011",
            ['m'] = "01010",
            ['p'] = "0100",
            ['y'] = "11100",
            ['a'] = "011",
            ['s'] = "1001",
            ['t'] = "1000",
            ['n'] = "101",
            ['i'] = "110",
            [' '] = "00"
        };
        var encoded = HuffmanTree.Encode(text, map);
        Assert.That(encoded, Is.EqualTo("10001111011111000101101111010100110101001001010001111010100100110000111110010010001010011110101111011110000110101001000111101111100010011101011110101"));
        Assert.Pass();

    }

    [Test]
    public void Test5_Decode()
    {
        var text = "the rain in spain stays mainly in the plain";
        var profile = HuffmanTree.Profile(text);
        var tree = HuffmanTree.BuildTree(profile);
        var map = HuffmanTree.CreateEncodingMap(tree);
        var encoded = HuffmanTree.Encode(text, map);
        var decoded = HuffmanTree.Decode(encoded, tree);
        Assert.That(decoded, Is.EqualTo("the rain in spain stays mainly in the plain"));
        Assert.Pass();

    }
}

#pragma warning restore CS8602  // Dereference of a possibly null reference.