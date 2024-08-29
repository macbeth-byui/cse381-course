// CSE 381 REPL 9A Solution
// Symmetric Key (Pad)

public static class SymmetricKey
{
    public static string EncryptDecrypt(string text, int key)
    {
        string result = "";
        foreach (char c in text)
        {
            var cInt = (int)c;
            var encrypt = cInt ^ key;
            result += (char)encrypt;
        }
        return result;
    }
    
}

class Program
{
    public static void Main(string[] args) 
    {
        var key = 42;
        var text = "The rain in spain stays mainly in the plain.";
        Console.WriteLine(text);
        var eText = SymmetricKey.EncryptDecrypt(text, key);
        Console.WriteLine(eText);
        var dText = SymmetricKey.EncryptDecrypt(eText, 42);
        Console.WriteLine(dText);      
    }
}