using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    public class Challenge05 : Challenge
    {
        private (Vector2Int, Vector2Int)[] Vents;
        public override void ParseInput()
        {
            Vents = new (Vector2Int, Vector2Int)[input.Length];

            for (int i = 0; i < input.Length; i++)
            {
                int[] n = Array.ConvertAll<string, int>(Regex.Split(input[i], @"\D+"), int.Parse);
                Vents[i] = (new Vector2Int(n[0], n[1]), new Vector2Int(n[2], n[3]));
            }
        }

        public override object Task1()
        {
            bool[] straightLines = new bool[Vents.Length];
            int maxX = 0;
            int maxY = 0;

            for (int i = 0; i < Vents.Length; i++)
            {
                (Vector2Int a, Vector2Int b) = Vents[i];
                maxX = Math.Max(maxX, a.x);
                maxX = Math.Max(maxX, b.x);
                maxY = Math.Max(maxY, a.y);
                maxY = Math.Max(maxY, b.y);
                if (a.x == b.x || a.y == b.y)
                {
                    straightLines[i] = true;
                }
            }

            int[,] grid = new int[maxX + 1, maxY + 1];

            for (int i = 0; i < straightLines.Length; i++)
            {
                if (!straightLines[i]) continue;
                AddToGrid(grid, Vents[i].Item1, Vents[i].Item2);
            }

            int count = 0;
            foreach (int i in grid)
            {
                if (i > 1) count++;
            }
            return count;
        }

        private void AddToGrid(int[,] grid, Vector2Int start, Vector2Int end)
        {
            Vector2Int direction = end - start;
            direction.x = Math.Sign(direction.x);
            direction.y = Math.Sign(direction.y);

            grid[start.y, start.x]++;

            while (start != end)
            {
                start += direction;
                grid[start.y, start.x]++;
            }
        }

        // tfw part 2 only takes 5 seconds because part 1 was general purpose enough...
        // If lines become a theme this season I'm making a class for them
        public override object Task2()
        {
            int maxX = 0;
            int maxY = 0;

            for (int i = 0; i < Vents.Length; i++)
            {
                (Vector2Int a, Vector2Int b) = Vents[i];
                maxX = Math.Max(maxX, a.x);
                maxX = Math.Max(maxX, b.x);
                maxY = Math.Max(maxY, a.y);
                maxY = Math.Max(maxY, b.y);
            }

            int[,] grid = new int[maxX + 1, maxY + 1];

            for (int i = 0; i < Vents.Length; i++)
            {
                AddToGrid(grid, Vents[i].Item1, Vents[i].Item2);
            }

            int count = 0;
            foreach (int i in grid)
            {
                if (i > 1) count++;
            }
            return count;
        }
    }
}
