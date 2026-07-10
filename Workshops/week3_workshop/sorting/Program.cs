// CSE 381 Workshop 3
// Custom Objects to Sort

using System.Text.Json;
using System.Text.Json.Serialization;

// Inform C# what we will be deserializing (a List of Book objects)
[JsonSerializable(typeof(List<Book>))]
internal partial class BookJsonContext : JsonSerializerContext
{
}

public class Book
{
    public required string title { get; set; }
    public required string author { get; set; }
    public required string language { get; set; }
    public required int year { get; set; }

    public override string ToString()
    {
        return $"{title,-50}{author,-28}{language,-18}{year,-5}";
    }

}

class Program
{
    static async Task<List<Book>?> Get_Books()
    {
        string url = "https://raw.githubusercontent.com/benoitvallon/100-best-books/master/books.json"; 
        using HttpClient client = new HttpClient();
        try
        {
            HttpResponseMessage response = await client.GetAsync(url);
            if (response.IsSuccessStatusCode)
            {
                string data = response.Content.ReadAsStringAsync().Result;
                // Deserialize into a List<Book>
                return JsonSerializer.Deserialize(data, BookJsonContext.Default.ListBook);
            }
            Console.WriteLine($"Error reading from API: {response.StatusCode}");
            return null;
        }
        catch (HttpRequestException e)
        {
            Console.WriteLine($"Error connecting to the API: {e.Message}");
            return null;
        }
    }

	// Driver Code
    public static async Task Main(string[] args)
    {
		// Get the books from the API.  Wait for the response
        var books = await Get_Books();
        if (books == null)
        {
            return;
        }

        // Do your sorting here.  The first one does nothing.
        var sorted_books = books.AsEnumerable();
		
		// Display the books
        foreach (var book in sorted_books)
        {
            Console.WriteLine(book);
        }
    }
}