using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2015
{
    public class Challenge18 : Challenge
    {
        private const char ON = '#';
        private const char OFF = '.';
        const int turns = 100;

        List<Vector2Int> neighbors = new List<Vector2Int>(Vector2Int.CardinalDirections);


        public override object Task1()
        {
            char[,] lights = new char[input.Length, input[0].Length];
            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input[0].Length; x++)
                {
                    lights[y, x] = input[y][x];
                }
            }

            neighbors.Add(new Vector2Int(1, 1));
            neighbors.Add(new Vector2Int(-1, -1));
            neighbors.Add(new Vector2Int(-1, 1));
            neighbors.Add(new Vector2Int(1, -1));

            HashSet<Vector2Int> changes = new HashSet<Vector2Int>();
            for (int _ = 0; _ < turns; _++)
            {
                Vector2Int pos = Vector2Int.Zero;
                for (pos.y = 0; pos.y < input.Length; pos.y++)
                {
                    for (pos.x = 0; pos.x < input[0].Length; pos.x++)
                    {
                        int onCount = 0;
                        foreach (Vector2Int v in neighbors)
                        {
                            Vector2Int n = v + pos;
                            if (n.x < 0 || n.y < 0 || n.y > input.Length - 1 || n.x > input[0].Length - 1) continue;
                            if (lights[n.y, n.x] == ON) onCount++;
                        }
                        if (lights[pos.y, pos.x] == ON && (onCount < 2 || onCount > 3)) changes.Add(pos);
                        else if (lights[pos.y, pos.x] == OFF && onCount == 3) changes.Add(pos);
                    }
                }
                foreach (Vector2Int c in changes)
                {
                    lights[c.y, c.x] = lights[c.y, c.x] == ON ? OFF : ON;
                }
                changes.Clear();
            }

            return lights.Count(ON);
        }

        public override object Task2()
        {
            char[,] lights = new char[input.Length, input[0].Length];
            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input[0].Length; x++)
                {
                    lights[y, x] = input[y][x];
                }
            }

            Vector2Int[] corners = new Vector2Int[] {
                new Vector2Int(0,0),
                new Vector2Int(0, input[0].Length - 1),
                new Vector2Int(input.Length - 1, 0),
                new Vector2Int(input.Length - 1, input[0].Length - 1),
             };

            foreach (Vector2Int v in corners) lights[v.y, v.x] = ON;

            HashSet<Vector2Int> changes = new HashSet<Vector2Int>();
            for (int _ = 0; _ < turns; _++)
            {
                Vector2Int pos = Vector2Int.Zero;
                for (pos.y = 0; pos.y < input.Length; pos.y++)
                {
                    for (pos.x = 0; pos.x < input[0].Length; pos.x++)
                    {
                        if (Array.IndexOf(corners, pos) != -1) continue;

                        int onCount = 0;
                        foreach (Vector2Int v in neighbors)
                        {
                            Vector2Int n = v + pos;
                            if (n.x < 0 || n.y < 0 || n.y > input.Length - 1 || n.x > input[0].Length - 1) continue;
                            if (lights[n.y, n.x] == ON) onCount++;
                        }
                        if (lights[pos.y, pos.x] == ON && (onCount < 2 || onCount > 3)) changes.Add(pos);
                        else if (lights[pos.y, pos.x] == OFF && onCount == 3) changes.Add(pos);
                    }
                }
                foreach (Vector2Int c in changes)
                {
                    lights[c.y, c.x] = lights[c.y, c.x] == ON ? OFF : ON;
                }
                changes.Clear();
            }

            return lights.Count(ON);
        }
    }
}
