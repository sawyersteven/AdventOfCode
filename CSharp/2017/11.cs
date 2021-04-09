using System;
using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2017
{
    public class Challenge11 : Challenge
    {
        // https://www.redblobgames.com/grids/hexagons/#coordinates-cube
        private Dictionary<string, Vector3Int> d3 = new Dictionary<string, Vector3Int>()
        {
            {"n", new Vector3Int(0, 1, -1)},
            {"s", new Vector3Int(0, -1, 1)},
            {"nw", new Vector3Int(-1, 1, 0)},
            {"ne", new Vector3Int(1, 0, -1)},
            {"se", new Vector3Int(1, -1, 0)},
            {"sw", new Vector3Int(-1, 0, 1)}
        };

        public override object Task1()
        {
            Vector3Int end = Vector3Int.Zero;
            foreach (string s in rawInput.Split(','))
            {
                end += d3[s];
            }
            return end.ManhattanDistance(Vector3Int.Zero) / 2;
        }

        public override object Task2()
        {
            Vector3Int current = Vector3Int.Zero;
            Vector3Int farthest = Vector3Int.Zero;
            long farthestDist = 0;
            foreach (string s in rawInput.Split(','))
            {
                current += d3[s];
                long d = current.ManhattanDistance(Vector3Int.Zero);
                if (d > farthestDist)
                {
                    farthestDist = d;
                    farthest = current;
                }
            }
            return farthest.ManhattanDistance(Vector3Int.Zero) / 2;
        }
    }
}