namespace AlgorithmLibTest;
using AlgorithmLib;
using NUnit.Framework;

[TestFixture]
public class ConvexHullTest
{
    [Test]
    public void Test1()
    {
        var points = new List<ConvexHull.Point>()
        {
            new(0, 0),
            new(4, 0),
            new(3, 1),
            new(1, 1),
            new(8, 8),
            new(3, 6),
            new(1, 4),
            new(1, 3),
            new(0, 4),
            new(0, 2),
            new(5.5, 7)
        };
        var hull = ConvexHull.GenerateHull(points);
        
        var expected = new List<ConvexHull.Point>()
        {
            new(0, 0),
            new(4, 0),
            new(8, 8),
            new(3, 6),
            new(0, 4),
            new(0, 0)
        };
        
        Assert.That(hull.Count, Is.EqualTo(expected.Count));

        for (var i = 0; i < hull.Count; i++)
        {
            Assert.That(hull[i].Equals(expected[i]), Is.True);
        }

        Assert.Pass();
    }
    
   
    
}