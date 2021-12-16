using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    using node = ValueTuple<Vector2Int, int>;

    public class Challenge15 : Challenge
    {
        int[,] map;
        public override void ParseInput()
        {
            map = Utils.InputToIntArray(input, true, 0);
        }

        public override object Task1()
        {
            return FindPath(map);
        }

        private int FindPath(int[,] map)
        {
            List<node> open = new List<node>(map.GetLength(0) * map.GetLength(1));
            HashSet<Vector2Int> closed = new HashSet<Vector2Int>();
            open.Add((new Vector2Int(1, 1), 0));
            Vector2Int end = new Vector2Int(map.GetLength(0) - 2, map.GetLength(1) - 2);

            while (open.Count > 0)
            {
                open.Sort((a, b) => (a.Item2 <= b.Item2) ? 1 : -1);
                node current = open[^1];
                open.RemoveAt(open.Count - 1);
                closed.Add(current.Item1);

                if (current.Item1 == end) return current.Item2;

                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector2Int nextPos = current.Item1 + v;
                    if (closed.Contains(nextPos) || map[nextPos.y, nextPos.x] == 0) continue;
                    int nextScore = current.Item2 + map[nextPos.y, nextPos.x];

                    int matchInd = findMatch(nextPos);
                    if (matchInd == -1)
                    {
                        open.Add((nextPos, nextScore));
                    }
                    else if (open[matchInd].Item2 > nextScore)
                    {
                        open[matchInd] = (nextPos, nextScore);
                    }
                }
            }
            return -1;

            int findMatch(Vector2Int p)
            {
                for (int i = 0; i < open.Count; i++)
                {
                    if (open[i].Item1 == p) return i;
                }
                return -1;
            }
        }


        private int[,] MultiplyMap(int[,] map)
        {
            const int mapMultiple = 5;
            int my = map.GetLength(0) - 2;
            int mx = map.GetLength(1) - 2;
            int[,] bigMap = new int[input.Length * mapMultiple + 2, input[0].Length * mapMultiple + 2];

            for (int yMult = 0; yMult < mapMultiple; yMult++)
            {
                for (int xMult = 0; xMult < mapMultiple; xMult++)
                {
                    for (int y = 0; y < my; y++)
                    {
                        for (int x = 0; x < mx; x++)
                        {
                            int v = map[y + 1, x + 1];
                            v += yMult + xMult;
                            if (v > 9) v %= 9;
                            bigMap[y + (yMult * my) + 1, x + (xMult * mx) + 1] = v;
                        }
                    }
                }
            }
            return bigMap;
        }

        public override object Task2()
        {
            return FindPath(MultiplyMap(map));
        }
    }
}
