using System;
using AdventOfCode;
using Grids;

namespace Advent2017
{
    public class Challenge03 : Challenge
    {
        int addr;
        public override object Task1()
        {
            addr = int.Parse(input[0]);

            int ringEnd = 1;
            int ringNum = 0;
            while (ringEnd < addr)
            {
                ringNum++;
                ringEnd += ringNum * 2 * 4;
            }

            int stepsToCardinal = int.MaxValue;
            for (int i = 0; i < 4; i++)
            {
                int d = Math.Abs(addr - (ringEnd - (ringNum * i * 2) - ringNum));
                if (d < stepsToCardinal) stepsToCardinal = d;
            }

            return ringNum + stepsToCardinal;
        }

        public override object Task2()
        {
            int[,] grid = new int[20, 20]; // arbitrary size, increase if neccessary
            grid[5, 5] = 1;

            Vector2Int current = new Vector2Int(6, 5);

            int direction = 1;
            Vector2Int lhs = Vector2Int.CardinalDirections[2];

            int num = 1;
            while (num < addr)
            {
                num = SumNeighbors(grid, current);
                grid[current.y, current.x] = num;

                if (grid[current.y + lhs.y, current.x + lhs.x] == 0)
                {
                    current += lhs;
                    direction = (direction + 1) % 4;
                    lhs = Vector2Int.CardinalDirections[(direction + 1) % 4];
                }
                else
                {
                    current += Vector2Int.CardinalDirections[direction];
                }
            }
            return num;
        }

        private int SumNeighbors(int[,] grid, Vector2Int center)
        {
            int s = 0;
            s += grid[center.y - 1, center.x];
            s += grid[center.y + 1, center.x];
            s += grid[center.y, center.x - 1];
            s += grid[center.y + 1, center.x - 1];
            s += grid[center.y - 1, center.x - 1];
            s += grid[center.y, center.x + 1];
            s += grid[center.y + 1, center.x + 1];
            s += grid[center.y - 1, center.x + 1];
            return s;
        }
    }
}
