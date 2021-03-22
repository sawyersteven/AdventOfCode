using AdventOfCode;
using System;
using System.Collections.Generic;
using Grids;

namespace Advent2018
{
    public class Challenge17 : Challenge
    {
        private const char clay = '#';
        private const char empty = default(char);
        private const char water = '~';
        private const char drop = '|';

        private char[,] grid;
        private Queue<Vector2Int> dropPoints = new Queue<Vector2Int>();
        private Queue<Vector2Int> floodPoints = new Queue<Vector2Int>();
        public override object Task1()
        {
            grid = MakeGrid();

            RunSimulation();

            return CountWater();
        }

        private void RunSimulation()
        {
            Vector2Int start = new Vector2Int();
            for (int i = 0; i < grid.GetLength(0); i++)
            {
                if (grid[0, i] == '+')
                {
                    start.x = i;
                    break;
                }
            }
            dropPoints.Enqueue(start);


            while (dropPoints.Count + floodPoints.Count > 0)
            {
                while (floodPoints.TryDequeue(out Vector2Int v))
                {
                    FloodFill(v);
                }

                FillDown(dropPoints.Dequeue());
            }
        }

        private long CountWater()
        {
            int y = 0;
            for (y = 0; y < grid.GetLength(0); y++)
            {
                if (Array.IndexOf(Utils.GetRow(grid, y), clay) != -1) break;
            }

            long waterCounter = 0;
            for (; y < grid.GetLength(0); y++)
            {
                for (int x = 0; x < grid.GetLength(1); x++)
                {
                    char c = grid[y, x];
                    if (c == water || c == drop) waterCounter++;
                }
            }
            return waterCounter;
        }

        private long CountWaterT2()
        {
            int y = 0;
            for (y = 0; y < grid.GetLength(0); y++)
            {
                if (Array.IndexOf(Utils.GetRow(grid, y), clay) != -1) break;
            }

            long waterCounter = 0;
            for (; y < grid.GetLength(0); y++)
            {
                for (int x = 0; x < grid.GetLength(1); x++)
                {
                    char c = grid[y, x];
                    if (c == water) waterCounter++;
                }
            }
            return waterCounter;
        }

        private void FloodFill(Vector2Int start)
        {
            bool dropFound = false;
            while (!dropFound)
            {
                grid[start.y, start.x] = drop;

                int y = start.y;
                int xLeft = start.x;
                int xRight = start.x;

                while (true)
                {
                    char next = grid[y, xLeft - 1];
                    if (next == clay) break;

                    xLeft--;
                    char nextBelow = grid[y + 1, xLeft];
                    if (nextBelow == drop) break;
                    if (nextBelow == empty)
                    {
                        dropFound = true;
                        dropPoints.Enqueue(new Vector2Int(xLeft, start.y));
                        break;
                    }
                }

                while (true)
                {
                    char next = grid[y, xRight + 1];
                    if (next == clay) break;

                    xRight++;
                    char below = grid[y + 1, xRight];
                    if (below == empty)
                    {
                        dropFound = true;
                        dropPoints.Enqueue(new Vector2Int(xRight, start.y));
                        break;
                    }
                }

                char fillChar = dropFound ? drop : water;
                for (; xLeft <= xRight; xLeft++)
                {
                    grid[y, xLeft] = fillChar;
                }
                if (dropFound) break;
                start.y--;
            }
        }

        private void FillDown(Vector2Int start)
        {
            if (grid[start.y + 1, start.x] != empty) return;

            while (grid[start.y + 1, start.x] == empty)
            {
                start.y += 1;
                grid[start.y, start.x] = drop;
                if (start.y + 1 == grid.GetLength(0) || grid[start.y + 1, start.x] == drop)
                {
                    return;
                }
            }
            floodPoints.Enqueue(start);
            return;
        }

        private char[,] MakeGrid()
        {
            List<Vector2Int> clayCells = new List<Vector2Int>();

            int xMin = int.MaxValue;
            int xMax = 0;
            int yMax = 0;

            foreach (string line in input)
            {
                if (line[0] == 'x')
                {
                    int x = int.Parse(line.Substring(2).Split(',')[0]);

                    string[] yrange = line.Split("y=")[1].Split("..");

                    int ymin = int.Parse(yrange[0]);
                    int ymax = int.Parse(yrange[1]);
                    for (; ymin <= ymax; ymin++)
                    {
                        clayCells.Add(new Vector2Int(x, ymin));
                    }
                }
                else if (line[0] == 'y')
                {
                    int y = int.Parse(line.Substring(2).Split(',')[0]);

                    string[] xrange = line.Split("x=")[1].Split("..");

                    int xmin = int.Parse(xrange[0]);
                    int xmax = int.Parse(xrange[1]);
                    for (; xmin <= xmax; xmin++)
                    {
                        clayCells.Add(new Vector2Int(xmin, y));
                    }
                }
            }

            foreach (Vector2Int c in clayCells)
            {
                if (c.x < xMin) xMin = c.x;
                if (c.x > xMax) xMax = c.x;
                if (c.y > yMax) yMax = c.y;
            }
            yMax++;

            xMin -= 1;

            int width = xMax - xMin + 1;

            char[,] grid = new char[yMax, width];

            foreach (Vector2Int c in clayCells)
            {
                grid[c.y, c.x - xMin] = clay;
            }

            grid[0, 500 - xMin] = '+';

            return grid;
        }

        public override object Task2()
        {
            return CountWaterT2();
        }
    }
}