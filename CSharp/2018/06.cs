using AdventOfCode;
using ExtensionMethods;
using System;

namespace Advent2018
{
    public class Challenge06 : Challenge
    {

        public override object Task1()
        {
            (int, int)[] points = new (int, int)[input.Length];

            int minX = int.MaxValue;
            int minY = int.MaxValue;
            int maxX = 0;
            int maxY = 0;

            for (int i = 0; i < input.Length; i++)
            {
                string[] parts = input[i].Split(", ");
                points[i] = (int.Parse(parts[0]), int.Parse(parts[1]));
                if (points[i].Item1 > maxX) maxX = points[i].Item1;
                if (points[i].Item1 < minX) minX = points[i].Item1;
                if (points[i].Item2 > maxY) maxY = points[i].Item2;
                if (points[i].Item2 < minY) minY = points[i].Item2;
            }

            int[] coverage = new int[points.Length];
            for (int x = minX; x < maxX + 1; x++)
            {
                for (int y = minY; y < maxY + 1; y++)
                {
                    bool solo = false;
                    int bestPoint = -1;
                    int shortestDist = int.MaxValue;
                    for (int i = 0; i < points.Length; i++)
                    {
                        (int, int) p = points[i];
                        int dist = Math.Abs(x - p.Item1) + Math.Abs(y - p.Item2);

                        if (dist == shortestDist)
                        {
                            solo = false;
                        }
                        else if (dist < shortestDist)
                        {
                            solo = true;
                            shortestDist = dist;
                            bestPoint = i;
                        }
                    }
                    if (solo)
                    {
                        coverage[bestPoint]++;
                    }
                }
            }
            return coverage.MaxVal();
        }

        public override object Task2()
        {
            (int, int)[] points = new (int, int)[input.Length];

            int minX = int.MaxValue;
            int minY = int.MaxValue;
            int maxX = 0;
            int maxY = 0;

            for (int i = 0; i < input.Length; i++)
            {
                string[] parts = input[i].Split(", ");
                points[i] = (int.Parse(parts[0]), int.Parse(parts[1]));
                if (points[i].Item1 > maxX) maxX = points[i].Item1;
                if (points[i].Item1 < minX) minX = points[i].Item1;
                if (points[i].Item2 > maxY) maxY = points[i].Item2;
                if (points[i].Item2 < minY) minY = points[i].Item2;
            }

            const int maxDist = 10000;
            int counter = 0;

            int[,] grid = new int[maxY - minY, maxX - minX];

            for (int x = minX; x < maxX + 1; x++)
            {
                for (int y = minY; y < maxY + 1; y++)
                {
                    int cellDistance = 0;
                    foreach ((int, int) p in points)
                    {
                        cellDistance += Math.Abs(x - p.Item1) + Math.Abs(y - p.Item2);
                    }
                    if (cellDistance < maxDist) counter++;

                }
            }
            return counter;
        }
    }
}