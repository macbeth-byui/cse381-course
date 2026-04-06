// CSE 381 REPL 1A Solution
// C# Primer

public static class Program 
{
    
    public static void Main (string[] args) 
    {
        Console.WriteLine ("Hello World");

        /***************************************************
         * Variables
         ***************************************************/

        // int a = 5;
        var a = 5;
        double b = 6.5;
        char c = 'a';
        string d = "CSE381";
        bool e = true;

        Console.WriteLine($"a = {a} b = {b} c = {c} d = {d} e = {e}");
        Console.WriteLine("===========================");

        /***************************************************
         * Division
         ***************************************************/

        var f = 42 / a;
        var g = 42 / b;
        Console.WriteLine($"f = {f} g = {g}");
        Console.WriteLine("===========================");

        /***************************************************
         * Loops
         ***************************************************/

        for (var i=0; i<10; i++) {
            Console.WriteLine(i);
        }
        Console.WriteLine("===========================");

        foreach (var i in Enumerable.Range(5,10)) {  // Start at 5 and go 10 values
            Console.WriteLine(i);
        }
        Console.WriteLine("===========================");

        /***************************************************
         * List Creation
         ***************************************************/

        var list1 = new List<int>();
        list1.Add(10);
        list1.Add(20);
        list1.Add(30);
        List<int> list2 = [-7, 5, -4, 1, 2, 4, 8, 5];
        List<int> list3 = Enumerable.Range(5,10).ToList();
        List<int> list4 = Enumerable.Repeat(13,10).ToList();

        foreach (var i in list4) {
            Console.WriteLine(i);
        }
        Console.WriteLine("===========================");

        /***************************************************
         * Accessing by Index
         ***************************************************/

        Console.WriteLine($"First: {list2[0]}");
        Console.WriteLine($"Last: {list2[^1]}");
        Console.WriteLine("===========================");

        /***************************************************
         * Slices
         ***************************************************/

        var list5 = list2[2..5];
        var list6 = list2[..5];
        var list7 = list2[2..];
        foreach (var i in list6) {
            Console.WriteLine(i);
        }
        Console.WriteLine("===========================");

        /***************************************************
         * Apply to each Element
         ***************************************************/

        var list8 = list2.Select(x => 2*x);
        var list9 = list2.Select(_ => 42);

        foreach (var i in list8) {
            Console.WriteLine(i);
        }
        Console.WriteLine("===========================");

        /***************************************************
         * Sorting
         ***************************************************/

        var list10 = list2.Order();
        var list11 = list2.OrderBy(x => x*x);
        List<(int, int)> list12 = [(-5,9), (-3,2), (4,7), (3,1), (-4,8)];
        var list13 = list12.OrderBy(x => x.Item1*x.Item1).ThenBy(x => x.Item2);

        foreach (var i in list13) {
            Console.WriteLine(i);
        }
        Console.WriteLine("===========================");

        /***************************************************
         * Dictionary Creation
         ***************************************************/

        var accounts = new Dictionary<string, int>() {
            {"bob",332},
            {"tim",153},
            {"sue",400}
        };

        Console.WriteLine(accounts["bob"]);
        Console.WriteLine("===========================");

        /***************************************************
         * Access and Modify
         ***************************************************/
       
        accounts["bob"] += 100;
        Console.WriteLine(accounts["bob"]);
        Console.WriteLine("===========================");

        /***************************************************
         * Contains Key
         ***************************************************/

        var name = "george";
        if (accounts.ContainsKey(name))
        {
            Console.WriteLine("Account Exists");
        }
        else
        {
            Console.WriteLine("Account does not Exist");
        }
        Console.WriteLine("===========================");

        /***************************************************
         * Loop Key/Value Pairs
         ***************************************************/

        foreach ((var key, var value) in accounts)
        {
            Console.WriteLine($"Key = {key} Value = {value}");
        }
        Console.WriteLine("===========================");
        
        /***************************************************
         * Functions with Generics
         ***************************************************/

        var max1 = Max(list2);
        var max2 = Max(["dog","cat","pig","cow","hamster","bird"]);

        Console.WriteLine(max1);
        Console.WriteLine(max2);
        Console.WriteLine("===========================");

        /***************************************************
         * Nullable Types
         ***************************************************/

        Console.WriteLine(AddOne(5));
        Console.WriteLine(AddOne(null));
    }

    // public static int Max(List<int> values)
    public static T Max<T>(List<T> values) where T : IComparable
    {
        if (values.Count == 0) {
            throw new Exception("List is empty");
        }
        var maxValue = values[0];
        for (var i=1; i<values.Count; i++)
        {
            // if (values[i] > maxValue)
            if (values[i].CompareTo(maxValue) > 0)
            {
                maxValue = values[i];
            }
        }
        return maxValue;
    }

    // public static int AddOne(int value)
    public static int AddOne(int? value)
    {
        // return value + 1;        
        return (value ?? 0) + 1;
    }


}