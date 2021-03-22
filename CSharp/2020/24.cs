using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    using Point = ValueTuple<int, int>;
    public class Challenge24 : Challenge
    {
        // maps to movements on a typical 2d grid
        private Dictionary<string, Point> directions = new Dictionary<string, Point>()
        {
            {"se", (0, -1)},
            {"nw", (0, 1)},
            {"ne", (1, 1)},
            {"w", (-1, 0)},
            {"e", (1, 0)},
            {"sw", (-1,-1)}
        };

        private Point MoveToTile(string moves)
        {
            Point location = (0, 0);
            for (int i = 0; i < moves.Length; i++)
            {
                string m = moves.Substring(i, 1);
                if (m == "s" || m == "n")
                {
                    m = moves.Substring(i, 2);
                    i++;
                }
                location.Item1 += directions[m].Item1;
                location.Item2 += directions[m].Item2;
            }
            return location;
        }

        private HashSet<Point> blackTiles = new HashSet<Point>();
        public override object Task1()
        {
            foreach (string line in input)
            {
                Point loc = MoveToTile(line);
                if (blackTiles.Contains(loc)) blackTiles.Remove(loc);
                else blackTiles.Add(loc);
            }
            return blackTiles.Count;
        }

        const int GridSize = 140; // mostly arbitrary
        const bool Black = true;
        const bool White = false;

        private Dictionary<Point, Point[]> adjacentTable = new Dictionary<Point, Point[]>();
        private Point[] adjOffsets = new Point[] { (0, 1), (1, 1), (1, 0), (0, -1), (-1, -1), (-1, 0) };
        private Point[] AdjacentTiles(int x, int y)
        {
            if (!adjacentTable.ContainsKey((x, y)))
            {
                List<Point> adj = new List<Point>(6);

                Point next;
                foreach (Point offset in adjOffsets)
                {
                    next.Item1 = x + offset.Item1;
                    next.Item2 = y + offset.Item2;
                    if (next.Item1 < 0 || next.Item2 < 0 || next.Item1 > GridSize - 1 || next.Item2 > GridSize - 1) continue;
                    adj.Add(next);
                }
                adjacentTable[(x, y)] = adj.ToArray();
            }
            return adjacentTable[(x, y)];
        }

        public override object Task2()
        {
            const int simulationDays = 100;

            bool[,] grid = new bool[GridSize, GridSize];
            const int zeroOffset = GridSize / 2;
            foreach (Point blackTile in blackTiles)
            {
                grid[blackTile.Item1 + zeroOffset, blackTile.Item2 + zeroOffset] = Black;
            }

            Queue<Point> changes = new Queue<Point>();

            for (int _ = 0; _ < simulationDays; _++)
            {
                Point p = (0, 0);
                for (int x = 0; x < GridSize; x++)
                {
                    p.Item1 = x;
                    for (int y = 0; y < GridSize; y++)
                    {
                        p.Item2 = y;
                        int blackNeighbors = 0;
                        foreach (Point n in AdjacentTiles(x, y))
                        {
                            if (grid[n.Item1, n.Item2]) blackNeighbors++;
                        }
                        if (grid[x, y] == Black)
                        {
                            if (blackNeighbors == 0 || blackNeighbors > 2) changes.Enqueue(p);
                        }
                        else
                        {
                            if (blackNeighbors == 2) changes.Enqueue(p);
                        }
                    }
                }

                Point change;
                while (true)
                {
                    if (!changes.TryDequeue(out change)) break;
                    bool t = grid[change.Item1, change.Item2];
                    grid[change.Item1, change.Item2] = !t;
                }
            }

            return CountBlackTiles(grid);
        }

        private int CountBlackTiles(bool[,] grid)
        {
            int count = 0;
            for (int x = 0; x < GridSize; x++)
            {
                for (int y = 0; y < GridSize; y++)
                {
                    if (grid[x, y] == Black) count++;
                }
            }
            return count;
        }
    }
}