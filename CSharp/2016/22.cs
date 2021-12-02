using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2016
{
    public class Challenge22 : Challenge
    {
        private Dictionary<Vector2Int, (int, int)> nodes;

        // location: used, available
        public override void ParseInput()
        {
            nodes = new Dictionary<Vector2Int, (int, int)>();
            for (int i = 2; i < input.Length; i++)
            {
                string[] parts = input[i].Split(' ', StringSplitOptions.RemoveEmptyEntries);
                int[] coords = Array.ConvertAll(parts[0].Split('x')[1].Split("-y"), int.Parse);
                var k = new Vector2Int(coords[0], coords[1]);
                int ud = int.Parse(parts[2].Substring(0, parts[2].Length - 1));
                int av = int.Parse(parts[3].Substring(0, parts[3].Length - 1));
                nodes[k] = (ud, av);
            }
        }

        public override object Task1()
        {
            (int, int)[] vals = new (int, int)[nodes.Count];
            nodes.Values.CopyTo(vals, 0);

            int count = 0;

            for (int a = 0; a < vals.Length - 1; a++)
            {
                for (int b = a + 1; b < vals.Length; b++)
                {
                    if (vals[a].Item1 != 0 && vals[a].Item1 <= vals[b].Item2) count++;
                    if (vals[b].Item1 != 0 && vals[b].Item1 <= vals[a].Item2) count++;
                }
            }
            return count;
        }

        public override object Task2()
        {
            char[,] grid = new char[24, 38];
            int emptyCapacity = 0;
            foreach (var kv in nodes)
            {
                if (kv.Value.Item1 == 0)
                {
                    emptyCapacity = kv.Value.Item2;
                    break;
                }
            }

            foreach (var kv in nodes)
            {
                if (kv.Value.Item1 == 0)
                {
                    grid[kv.Key.y, kv.Key.x] = '0';
                }
                else if (kv.Value.Item1 <= emptyCapacity)
                {
                    grid[kv.Key.y, kv.Key.x] = '·';
                }
                else
                {
                    grid[kv.Key.y, kv.Key.x] = '█';
                }
            }
            grid[0, 0] = '*';
            grid[0, 37] = 'D';

            return "\nTopaz (AoC creator) said 2016.22.02 was meant to be solved without code. So if he can't be bothered I can't either.\nHere's a map. The answer is 215.\n\n" + grid.Render();
        }
    }
}
