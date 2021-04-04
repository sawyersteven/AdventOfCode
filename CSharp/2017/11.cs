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
            return AStar(Vector3Int.Zero, end);
        }

        //A* is probably overkill but whatever
        private int AStar(Vector3Int start, Vector3Int end)
        {
            List<Vector3Int[]> open = new List<Vector3Int[]>();
            HashSet<Vector3Int> closed = new HashSet<Vector3Int>();
            open.Add(new Vector3Int[] { Vector3Int.Zero });

            while (open.Count > 0)
            {
                open.Sort((a, b) => b[^1].ManhattanDistance(end) - (a[^1].ManhattanDistance(end)));

                Vector3Int[] path = open[^1];
                open.RemoveAt(open.Count - 1);
                closed.Add(path[^1]);

                foreach (Vector3Int v in d3.Values)
                {
                    Vector3Int next = path[^1] + v;

                    if (closed.Contains(next)) continue;
                    if (next == end) return path.Length;

                    Vector3Int[] np = new Vector3Int[path.Length + 1];
                    Array.Copy(path, np, path.Length);
                    np[^1] = next;

                    open.Add(np);
                    closed.Add(next);
                }
            }
            return -1;
        }

        public override object Task2()
        {
            Vector3Int current = Vector3Int.Zero;
            Vector3Int farthest = Vector3Int.Zero;
            int farthestDist = 0;
            foreach (string s in rawInput.Split(','))
            {
                current += d3[s];
                int d = current.ManhattanDistance(Vector3Int.Zero);
                if (d > farthestDist)
                {
                    farthestDist = d;
                    farthest = current;
                }
            }
            return AStar(Vector3Int.Zero, farthest);
        }
    }
}