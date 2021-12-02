using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2018
{
    public class Challenge25 : Challenge
    {
        private const int MinDist = 3;

        public int ManhattanDistance(int[] a, int[] b)
        {
            return Math.Abs(a[0] - b[0]) + Math.Abs(a[1] - b[1]) + Math.Abs(a[2] - b[2]) + Math.Abs(a[3] - b[3]);
        }

        List<int[]> ogPoints;
        public override void ParseInput()
        {
            ogPoints = new List<int[]>(input.Length);
            for (int i = 0; i < input.Length; i++)
            {
                ogPoints.Add(Array.ConvertAll(input[i].Split(','), int.Parse));
            }
        }

        public override object Task1()
        {
            List<int[]> points = ogPoints.Duplicate();

            int total = 0;
            while (true)
            {
                List<int[]> constellation = new List<int[]>();
                int[] seed = points[^1];
                constellation.Add(seed);
                points.RemoveAt(points.Count - 1);

                bool changes = true;
                while (changes)
                {
                    changes = false;
                    for (int i = 0; i < constellation.Count; i++)
                    {
                        for (int j = 0; j < points.Count; j++)
                        {
                            if (ManhattanDistance(points[j], constellation[i]) <= MinDist)
                            {
                                constellation.Add(points[j]);
                                points.RemoveAt(j);
                                j--;
                                changes = true;
                            }
                        }
                    }
                }
                total++;
                if (points.Count == 0) break;
            }
            return total;
        }

        public override object Task2()
        {
            return "*";
        }
    }
}