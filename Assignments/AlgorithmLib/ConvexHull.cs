/* CSE 381 - Convex Hull (Graham Scan)
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W11 Prove: Assignment in Canvas for detailed instructions.
*/

namespace AlgorithmLib;

public static class ConvexHull
{
    /* Valid angle types to be used in the code */
    public enum Angle
    {
        Convex,
        Concave,
        Colinear
    }
    
    /* Representation of a 2D point with support
     * for comparing 2 points for equivalence.
    */
    public class Point
    {
        public readonly double X;
        public readonly double Y;

        // Used for comparison of doubles
        public static double TOLERANCE = 0.001;

        public Point(double x, double y)
        {
            X = x;
            Y = y;
        }

        /* This function supports testing */
        public bool Equals(Point point)
        {
            return Math.Abs(X - point.X) < TOLERANCE &&
                   Math.Abs(Y - point.Y) < TOLERANCE;
        }
    }

    /* Determine if 3 points form a convex, concave, or
     * co-linear angle.
     *
     *  Inputs:
     *     a - Point 1
     *     b - Point 2
     *     c - Point 3
     *  Outputs:
     *     Return Angle.Convex, Angle.Concave, or Angle.Colinear
     */
    public static Angle Orientation(Point a, Point b, Point c)
    {
        return Angle.Convex; 
    }

    /* Determine the angle of a point relative to an anchor point.
     *
     *  Inputs:
     *     anchor - Anchor point
     *     point - Other point
     *  Outputs:
     *     Return angle in radians
     */
    public static double GetAngle(Point anchor, Point point)
    {
        return 0.0;
    }

    /* Determine the distance from an anchor point to another point
     *
     *  Inputs:
     *     anchor - Anchor point
     *     point - Other point
     *  Outputs:
     *     Return distance
     */
    public static double GetDist(Point anchor, Point point)
    {
        return 0.0;
    }

    /* General a Convex Hull from a list of points.
     *
     *  Inputs:
     *     points - List of points to create hull around
     *  Outputs:
     *     Return list of points in the hull
     *
     * NOTE: If no hull exists, then return an empty list.
     */
    public static List<Point> GenerateHull(List<Point> points)
    {
        
        return new List<Point>();
    }
}