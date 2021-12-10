using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    public class Challenge09 : Challenge
    {
        int[,] map;
        List<Vector2Int> lows;

        public override void ParseInput()
        {
            map = new int[input.Length + 2, input[0].Length + 2];

            map.Fill(9);

            int y = 1;
            int yEnd = input.Length + 1;
            int x = 1;
            int xEnd = input[0].Length + 1;

            for (int gy = 0; y < yEnd; y++, gy++)
            {
                int gx = 0;
                for (gx = 0, x = 1; x < xEnd; x++, gx++)
                {
                    map[y, x] = (int)char.GetNumericValue(input[gy][gx]);
                }
            }
        }

        public override object Task1()
        {
            lows = new List<Vector2Int>();
            int sum = 0;
            Vector2Int current = new Vector2Int(1, 1);
            for (current.x = 1; current.x < map.GetLength(0) - 1; current.x++)
            {
                for (current.y = 1; current.y < map.GetLength(1) - 1; current.y++)
                {
                    int c = map[current.x, current.y];
                    if (c < map[current.x + 1, current.y] && c < map[current.x, current.y + 1] && c < map[current.x - 1, current.y] && c < map[current.x, current.y - 1])
                    {
                        lows.Add(current);
                        sum += c + 1;
                    }
                }
            }
            return sum;
        }

        public override object Task2()
        {
            int[] s = new int[3];
            foreach (Vector2Int coord in lows)
            {
                int b = BasinSize(coord);

                for (int i = 2; i >= 0; i--)
                {
                    if (b > s[i])
                    {
                        for (int j = 0; j < i; j++)
                        {
                            s[j] = s[j + 1];
                        }
                        s[i] = b;
                        break;
                    }
                }
            }
            return s[0] * s[1] * s[2];

            // 1135260 @ 12.8ms
        }

        private int BasinSize(Vector2Int lowCoord)
        {
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>();
            Queue<Vector2Int> q = new Queue<Vector2Int>();
            q.Enqueue(lowCoord);
            visited.Add(lowCoord);

            while (q.Count > 0)
            {
                Vector2Int current = q.Dequeue();
                foreach (Vector2Int n in Vector2Int.CardinalDirections)
                {
                    Vector2Int next = current + n;
                    if (visited.Contains(next) || map[next.x, next.y] == 9) continue;
                    visited.Add(next);
                    q.Enqueue(next);
                }
            }
            return visited.Count;
        }
    }
}
