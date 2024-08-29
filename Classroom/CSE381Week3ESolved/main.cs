// CSE 381 REPL 3E Solution
// Custom Objects to Sort

using System;

public class Home : IComparable
{
    private string _address;
    private float _value;
    private int _bedrooms;
    private int _bathrooms;

    public Home(string address, float value, int bedrooms, int bathrooms)
    {
        _address = address;
        _value = value;
        _bedrooms = bedrooms;
        _bathrooms = bathrooms;
    }

    public override string ToString()
    {
        var result = $"{_address} ({_value}) - BED: {_bedrooms} BATH: {_bathrooms}";
        return result;
    }

    public int CompareTo(Object? homeObj)
    {
        if (homeObj == null)
        {
            return 1;
        }
        var home = homeObj as Home;
        if (home == null)
        {
            throw new ArgumentException("Object is not a Home");
        }
        return _value.CompareTo(home._value);
    }
}

class Program
{
    public static void Main(string[] args)
    {
        var homes = new List<Home>()
        {
            new Home("123 Elm St", 500000, 4, 2),
            new Home("1882 Fortune Rd", 1000000, 8, 10),
            new Home("99 Simple Way", 100000, 3, 2),
            new Home("000 Lost Blvd", 20000, 1, 0)
        };

        homes.Sort();

        foreach (var home in homes)
        {
            Console.WriteLine(home);
        }
        
    }
}