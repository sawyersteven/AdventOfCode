using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2016
{
    public class Challenge24 : Challenge
    {
        HashSet<Vector2Int> walls;
        Dictionary<(Vector2Int, Vector2Int), int> distanceCache;
        Vector2Int start;
        List<Vector2Int> targets;

        private Dictionary<(Vector2Int, Vector2Int), int> CalcDistances(List<Vector2Int> points)
        {
            Dictionary<(Vector2Int, Vector2Int), int> paths = new Dictionary<(Vector2Int, Vector2Int), int>();
            foreach (Vector2Int a in points)
            {
                foreach (Vector2Int b in points)
                {
                    if (a == b) continue;
                    paths[(a, b)] = Distance(a, b);
                }
            }
            return paths;
        }

        private int Distance(Vector2Int a, Vector2Int b)
        {
            Queue<(Vector2Int, int)> q = new Queue<(Vector2Int, int)>();
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>();
            q.Enqueue((a, 0));

            while (q.Count > 0)
            {
                (Vector2Int current, int dist) = q.Dequeue();
                if (current == b) return dist;
                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector2Int step = current + v;
                    if (visited.Contains(step) || walls.Contains(step)) continue;
                    visited.Add(step);
                    q.Enqueue((step, dist + 1));
                }
            }
            throw new System.Exception();
        }

        public override object Task1()
        {
            targets = new List<Vector2Int>();
            walls = new HashSet<Vector2Int>();
            char[,] map = new char[input.Length, input[0].Length];
            start = Vector2Int.Zero;

            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input[0].Length; x++)
                {
                    if (input[y][x] == '#') walls.Add(new Vector2Int(x, y));
                    else if (input[y][x] > '0' && input[y][x] <= '9') targets.Add(new Vector2Int(x, y));
                    else if (input[y][x] == '0') start = new Vector2Int(x, y);
                }
            }

            targets.Add(start);
            distanceCache = CalcDistances(targets);
            targets.Remove(start);

            int minDist = int.MaxValue;
            foreach (IList<Vector2Int> path in Permutator.Permutate<Vector2Int>(targets))
            {
                int dist = 0;

                dist += distanceCache[(start, path[0])];

                for (int i = 0; i < path.Count - 1; i++)
                {
                    var k = (path[i], path[i + 1]);
                    if (!distanceCache.ContainsKey(k)) k = (k.Item2, k.Item1);
                    dist += distanceCache[k];
                    if (dist > minDist) break;
                }
                if (dist < minDist) minDist = dist;
            }
            return minDist;
        }

        public override object Task2()
        {
            int minDist = int.MaxValue;
            foreach (IList<Vector2Int> path in Permutator.Permutate<Vector2Int>(targets))
            {
                int dist = 0;

                dist += distanceCache[(start, path[0])];

                for (int i = 0; i < path.Count - 1; i++)
                {
                    var k = (path[i], path[i + 1]);
                    if (!distanceCache.ContainsKey(k)) k = (k.Item2, k.Item1);
                    dist += distanceCache[k];
                    if (dist > minDist) break;
                }
                dist += distanceCache[(start, path[^1])];
                if (dist < minDist) minDist = dist;
            }
            return minDist;
        }
    }
}
