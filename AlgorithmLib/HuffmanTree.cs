namespace AlgorithmLib;

public static class HuffmanTree
{
    public class Node
    {
        public char Letter { get; set; }
        public float Count { get; set; }
        
        public Node? Left;
        public Node? Right;
    }

    public static Dictionary<char,int> Profile(String text)
    {
        // ADD CODE HERE AND FIX RETURN STATEMENT
        return new Dictionary<char, int>();
    }
    public static Node BuildTree(Dictionary<char, int> profile)
    {
        // ADD CODE HERE AND FIX RETURN STATEMENT
        return new Node
        {
            Count = 0,
            Letter = '\0',
            Left = null,
            Right = null
        };
    }

    public static Dictionary<char, string> CreateEncodingMap(Node root)
    {
        // ADD CODE HERE AND FIX RETURN STATEMENT
        return new Dictionary<char, string>();
    }

    public static string Encode(string text, Dictionary<char, string> map)
    {
        // ADD CODE HERE AND FIX RETURN STATEMENT
        return "";
    }

    public static string Decode(string text, Node tree)
    {
        // ADD CODE HERE AND FIX RETURN STATEMENT
        return "";
    }
}