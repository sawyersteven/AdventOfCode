using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2019
{
    public class Challenge18 : Challenge
    {

        char[,] grid;
        public override object Task1()
        {
            Dictionary<char, Vector2Int> keys = new Dictionary<char, Vector2Int>();
            Dictionary<char, Vector2Int> doors = new Dictionary<char, Vector2Int>();

            grid = new char[input.Length, input[0].Length];
            for (int r = 0; r < input.Length; r++)
            {
                for (int c = 0; c < input[0].Length; c++)
                {
                    grid[r, c] = input[r][c];
                    if (grid[r, c] >= 'A')
                    {
                        if (grid[r, c] >= 'a')
                        {
                            keys[grid[r, c]] = new Vector2Int(c, r);
                        }
                        else
                        {
                            doors[grid[r, c]] = new Vector2Int(c, r);
                        }
                    }
                }
            }
            Utils.Print2DArray(grid);
            return null;
        }

        public override object Task2()
        {

            return null;
        }
    }
}