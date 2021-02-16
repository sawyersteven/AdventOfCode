using System;

namespace Advent2019
{
    using System.Collections.Generic;
    using System.Linq;
    using Point = ValueTuple<int, int>;
    using Point3 = ValueTuple<int, int, int>;

    public class Challenge03 : Challenge
    {
        private List<Point> TraceWire(string inputLine)
        {
            List<Point> points = new List<Point>();

            Point current = (0, 0);
            foreach (string move in inputLine.Split(','))
            {
                char dir = move[0];
                int dist = int.Parse(move.Substring(1));
                if (dir == 'R')
                {
                    for (int i = 0; i < dist; i++)
                    {
                        current.Item2 += 1;
                        points.Add(current);
                    }
                }
                else if (dir == 'L')
                {
                    for (int i = 0; i < dist; i++)
                    {
                        current.Item2 -= 1;
                        points.Add(current);
                    }
                }
                else if (dir == 'U')
                {
                    for (int i = 0; i < dist; i++)
                    {
                        current.Item1 += 1;
                        points.Add(current);
                    }
                }
                else if (dir == 'D')
                {
                    for (int i = 0; i < dist; i++)
                    {
                        current.Item1 -= 1;
                        points.Add(current);
                    }
                }
            }
            return points;
        }

        private List<Point> intersections;
        private List<Point> w1Path;
        private List<Point> w2Path;
        public override object Task1()
        {
            w1Path = TraceWire(input[0]);
            w2Path = TraceWire(input[1]);
            intersections = new List<Point>();

            HashSet<Point> set = new HashSet<Point>(w1Path);
            foreach (Point element in w2Path)
            {
                if (set.Remove(element)) intersections.Add(element);
            }

            int minDist = int.MaxValue;
            foreach (Point p in intersections)
            {
                int d = Math.Abs(p.Item1) + Math.Abs(p.Item2);
                if (d < minDist) minDist = d;
            }
            return minDist;
        }

        public override object Task2()
        {
            int minDist = int.MaxValue;
            foreach (Point intersection in intersections)
            {
                int d = w1Path.IndexOf(intersection) + w2Path.IndexOf(intersection) + 2;
                if (d < minDist) minDist = d;
            }

            return minDist;
        }
    }
}