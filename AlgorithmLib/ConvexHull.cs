using System.Collections;

namespace AlgorithmLib;

public static class ConvexHull
{
    private const double DoubleTolerance = 0.001;

    public class Point
    {
        public double X;
        public double Y;

        public Point(double x, double y)
        {
            X = x;
            Y = y;
        }

        public bool Equals(Point point)
        {
            return Math.Abs(X - point.X) < DoubleTolerance && 
                   Math.Abs(Y - point.Y) < DoubleTolerance;
        }
    }

    private static int Orientation(Point a, Point b, Point c)
    {
        // ADD CODE HERE AND FIX RETURN STATEMENT
        return 0;
    }
    
    // If needed, you can create more private functions to be used by the 
    // GenerateHull function below.

    public static List<Point> GenerateHull(List<Point> points)
    {
        // ADD CODE HERE AND FIX RETURN STATEMENT
        return new List<Point>();
    }
}