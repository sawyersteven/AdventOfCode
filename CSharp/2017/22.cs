using System;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2017
{
    public class Challenge22 : Challenge
    {
        private const char Clean = '.';
        private const char Infected = '#';
        private const char Weak = 'W';
        private const char Flagged = 'F';


        private char[,] ParseInput(int padding)
        {
            int h = input.Length;
            int w = input[0].Length;
            char[,] grid = new char[h + (padding * 2), w + (padding * 2)];
            grid.Fill(Clean);

            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    grid[y + padding, x + padding] = input[y][x];
                }
            }

            return grid;
        }

        public override object Task1()
        {
            const int turns = 10000;
            int direction = 2;

            char[,] grid = ParseInput(1000);

            Vector2Int position = new Vector2Int((grid.GetLength(1) - 1) / 2, (grid.GetLength(0) - 1) / 2);

            int infectionsCreated = 0;
            for (int _ = 0; _ < turns; _++)
            {
                char current = grid[position.y, position.x];
                if (current == Infected)
                {
                    direction = (direction + 3) % 4;
                    grid[position.y, position.x] = Clean;
                }
                else
                {
                    direction = (direction + 1) % 4;
                    grid[position.y, position.x] = Infected;
                    infectionsCreated++;
                }
                position += Vector2Int.CardinalDirections[direction];
            }
            return infectionsCreated;
        }




        public override object Task2()
        {
            const int turns = 10000000;
            int direction = 2;

            char[,] grid = ParseInput(1000);

            Vector2Int position = new Vector2Int((grid.GetLength(1) - 1) / 2, (grid.GetLength(0) - 1) / 2);

            int infectionsCreated = 0;
            for (int _ = 0; _ < turns; _++)
            {
                char current = grid[position.y, position.x];
                switch (current)
                {
                    case Infected:
                        grid[position.y, position.x] = Flagged;
                        direction = (direction + 3) % 4;
                        break;
                    case Flagged:
                        grid[position.y, position.x] = Clean;
                        direction = (direction + 2) % 4;
                        break;
                    case Clean:
                        grid[position.y, position.x] = Weak;
                        direction = (direction + 1) % 4;
                        break;
                    case Weak:
                        grid[position.y, position.x] = Infected;
                        infectionsCreated++;
                        break;
                }
                position += Vector2Int.CardinalDirections[direction];
            }
            return infectionsCreated;
        }
    }
}
