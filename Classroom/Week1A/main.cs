// CSE 381 REPL 1A
// C# Primer

public static class Program 
{
    
    public static void Main (string[] args) 
    {
        Console.WriteLine ("Hello World");

        /***************************************************
         * Variables
         ***************************************************/


        // Console.WriteLine($"a = {a} b = {b} c = {c} d = {d} e = {e}");
        // Console.WriteLine("===========================");



        /***************************************************
         * Division
         ***************************************************/



        // Console.WriteLine($"f = {f} g = {g}");
        // Console.WriteLine("===========================");



        /***************************************************
         * Loops
         ***************************************************/


        // Console.WriteLine("===========================");


        // Console.WriteLine("===========================");



        /***************************************************
         * List Creation
         ***************************************************/


        // foreach (var i in list1) {
        //     Console.WriteLine(i);
        // }
        // Console.WriteLine("===========================");



        /***************************************************
         * Accessing by Index
         ***************************************************/


        // Console.WriteLine($"First: {list2[]}");
        // Console.WriteLine($"Last: {list2[]}");
        // Console.WriteLine("===========================");



        /***************************************************
         * Slices
         ***************************************************/



        // foreach (var i in list5) {
        //     Console.WriteLine(i);
        // }
        // Console.WriteLine("===========================");



        /***************************************************
         * Apply to each Element
         ***************************************************/


        // foreach (var i in list8) {
        //     Console.WriteLine(i);
        // }
        // Console.WriteLine("===========================");



        /***************************************************
         * Sorting
         ***************************************************/



        // List<(int, int)> list12 = [(-5,9), (-3,2), (4,7), (3,1), (-4,8)];

        // foreach (var i in list10) {
        //     Console.WriteLine(i);
        // }
        // Console.WriteLine("===========================");



        /***************************************************
         * Dictionary Creation
         ***************************************************/


        var accounts = new Dictionary<string, int>() {
            {"bob",332},
            {"tim",153},
            {"sue",400}
        };

        // Console.WriteLine(accounts[]);
        // Console.WriteLine("===========================");



        /***************************************************
         * Access and Modify
         ***************************************************/
       

        // Console.WriteLine(accounts["bob"]);
        // Console.WriteLine("===========================");



        /***************************************************
         * Contains Key
         ***************************************************/


        // var name = "george";
        // if (accounts)
        // {
        //     Console.WriteLine("Account Exists");
        // }
        // else
        // {
        //     Console.WriteLine("Account does not Exist");
        // }
        // Console.WriteLine("===========================");



        /***************************************************
         * Loop Key/Value Pairs
         ***************************************************/



            // Console.WriteLine($"Key = {key} Value = {value}");

        // Console.WriteLine("===========================");



        /***************************************************
         * Functions with Generics
         ***************************************************/


        // var max1 = Max(list2);
        // var max2 = Max(["dog","cat","pig","cow","hamster","bird"]);

        // Console.WriteLine(max1);
        // Console.WriteLine(max2);
        // Console.WriteLine("===========================");


        /***************************************************
         * Nullable Types
         ***************************************************/


        // Console.WriteLine(AddOne(5));
        // Console.WriteLine(AddOne(null));


    }

    public static int AddOne(int value)
    {
        return value + 1;        
    }

}