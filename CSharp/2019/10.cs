using AdventOfCode;
using Grids;
using System;

namespace Advent2019
{
    using System.Collections.Generic;
    using System.Linq;

    public class Challenge10 : Challenge
    {
        private Vector2Int stationLocation;
        private List<Vector2Int> Asteroids = new List<Vector2Int>();
        public override object Task1()
        {
            Asteroids = new List<Vector2Int>();

            char[][] grid = new char[input.Length][];
            for (int y = 0; y < input.Length; y++)
            {
                grid[y] = new char[input[y].Length];
                for (int x = 0; x < input[y].Length; x++)
                {
                    grid[y][x] = input[y][x];
                    if (grid[y][x] == '#') Asteroids.Add(new Vector2Int(x, y));
                }
            }

            int maxCount = 0;
            HashSet<Vector2Int> consumedSlopes = new HashSet<Vector2Int>();
            foreach (Vector2Int a in Asteroids)
            {
                foreach (Vector2Int b in Asteroids)
                {
                    if (a == b) continue;
                    Vector2Int slope = b - a;
                    consumedSlopes.Add(slope / GCD(slope.x, slope.y));
                }
                if (consumedSlopes.Count > maxCount)
                {
                    maxCount = consumedSlopes.Count;
                    stationLocation = a;
                }
                consumedSlopes.Clear();
            }

            return maxCount;
        }
        private Vector2Int North = new Vector2Int(0, 1);
        public override object Task2()
        {
            List<(double, int, Vector2Int)> relativePositions = new List<(double, int, Vector2Int)>();

            foreach (Vector2Int other in Asteroids)
            {
                if (other == stationLocation) continue;
                int dist = Math.Abs(other.x - stationLocation.x) + Math.Abs(other.y - stationLocation.y);
                double angle = (360 - (stationLocation - other).AngleTo(North)) % 360;
                relativePositions.Add((angle, dist, other));
            }
            relativePositions = relativePositions.OrderBy(x => x.Item1).ThenBy(x => x.Item2).ToList();


            int removed = 0;
            int current = 0;
            while (true)
            {
                double angle = relativePositions[current].Item1;
                removed++;
                if (removed == 200) break;
                relativePositions.RemoveAt(current);
                while (relativePositions[current].Item1 == angle)
                {
                    current++;
                    current %= relativePositions.Count;
                }
            }

            Vector2Int answer = relativePositions[current].Item3;
            return answer.x * 100 + answer.y;
        }

        private int GCD(int a, int b)
        {
            while (b != 0)
            {
                int tmp = b;
                b = a % b;
                a = tmp;
            }
            return Math.Abs(a);
        }
    }
}