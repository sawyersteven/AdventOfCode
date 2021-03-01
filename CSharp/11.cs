using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge11 : Challenge
    {
        private const int gridSize = 100;
        private const char black = '0';
        private const char white = '1';

        private Vector2Int[] directions = new Vector2Int[] {
                new Vector2Int(0, -1),
                new Vector2Int(1, 0),
                new Vector2Int(0, 1),
                new Vector2Int(-1, 0)
            };

        public override object Task1()
        {
            char[,] grid = new char[gridSize, gridSize];
            for (int y = 0; y < gridSize; y++)
            {
                for (int x = 0; x < gridSize; x++)
                {
                    grid[y, x] = black;
                }
            }

            return RunBot(grid);
        }

        private int RunBot(char[,] grid)
        {
            IntCode.Emulator ICE = new IntCode.Emulator();
            long[] program = IntCode.Tools.ParseCode(input[0]);
            var response = ICE.Boot(program);

            long dir = 0;
            Vector2Int loc = new Vector2Int(gridSize / 2, gridSize / 2);

            HashSet<Vector2Int> paintedTiles = new HashSet<Vector2Int>();
            for (int i = 0; ; i++)
            {
                ICE.QueueInput((long)char.GetNumericValue(grid[loc.y, loc.x]));
                response = ICE.Run();
                if (response.Item1 != IntCode.ExitCode.OutputDelivery) break;

                grid[loc.y, loc.x] = response.Item2.ToString()[0];
                paintedTiles.Add(loc);

                response = ICE.Run();
                long dirDelta = response.Item2 == 1 ? 1 : -1;

                dir = (dir + dirDelta + directions.Length) % directions.Length;
                loc += directions[dir];
            }
            return paintedTiles.Count;
        }

        public override object Task2()
        {
            char[,] grid = new char[gridSize, gridSize];
            for (int y = 0; y < gridSize; y++)
            {
                for (int x = 0; x < gridSize; x++)
                {
                    grid[y, x] = black;
                }
            }

            grid[gridSize / 2, gridSize / 2] = white;

            RunBot(grid);

            for (int i = 0; i < gridSize; i++)
            {
                char[] c = new char[gridSize];
                for (int j = 0; j < gridSize; j++)
                {
                    c[j] = grid[i, j] == '0' ? ' ' : '#';
                }
                string s = string.Join("", c);
                if (string.IsNullOrWhiteSpace(s)) continue;
                Console.WriteLine(s);
            }
            return null;
        }
    }
}