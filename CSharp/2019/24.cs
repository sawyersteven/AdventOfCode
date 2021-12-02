using System;
using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2019
{
    public class Challenge24 : Challenge
    {
        private const bool Bug = true;
        private const bool Empty = false;
        private const int GridSize = 5;

        private bool[,] ogGrid;
        public override void ParseInput()
        {
            ogGrid = new bool[GridSize, GridSize];
            for (int y = 0; y < GridSize; y++)
            {
                for (int x = 0; x < GridSize; x++)
                {
                    ogGrid[y, x] = input[y][x] == '#';
                }
            }
        }

        private Dictionary<Vector2Int, Vector2Int[]> adjacentTable = new Dictionary<Vector2Int, Vector2Int[]>();
        private Vector2Int[] AdjacentTiles(Vector2Int center)
        {
            if (!adjacentTable.ContainsKey(center))
            {
                List<Vector2Int> adj = new List<Vector2Int>(6);

                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector2Int next = center + v;
                    if (next.x < 0 || next.y < 0 || next.x > GridSize - 1 || next.y > GridSize - 1) continue;
                    adj.Add(next);
                }
                adjacentTable[center] = adj.ToArray();
            }
            return adjacentTable[center];
        }

        private long BioRating(bool[,] grid)
        {
            long r = 0;
            long v = 1;
            for (int y = 0; y < GridSize; y++)
            {
                for (int x = 0; x < GridSize; x++)
                {
                    if (grid[y, x] == Bug)
                    {
                        r += v;
                    }
                    v <<= 1;
                }
            }
            return r;
        }

        public override object Task1()
        {
            bool[,] grid = (bool[,])ogGrid.Clone();

            HashSet<long> bioRatings = new HashSet<long>();

            Vector2Int current = new Vector2Int();

            HashSet<Vector2Int> changes = new HashSet<Vector2Int>();
            while (true)
            {
                changes.Clear();
                for (int y = 0; y < input.Length; y++)
                {
                    current.y = y;
                    for (int x = 0; x < input[0].Length; x++)
                    {
                        current.x = x;
                        int adjBugs = 0;
                        foreach (Vector2Int adj in AdjacentTiles(current))
                        {
                            if (grid[adj.y, adj.x] == Bug) adjBugs++;
                        }

                        if (grid[current.y, current.x] == Bug && adjBugs != 1) changes.Add(current);
                        else if (grid[current.y, current.x] == Empty && (adjBugs == 1 || adjBugs == 2)) changes.Add(current);

                    }
                }
                foreach (Vector2Int v in changes)
                {
                    grid[v.y, v.x] ^= true;
                }

                long br = BioRating(grid);
                if (bioRatings.Contains(br)) return br;
                bioRatings.Add(br);
            }
        }

        private HashSet<Vector3Int> ParseInput2()
        {
            HashSet<Vector3Int> bugs = new HashSet<Vector3Int>();
            for (int y = 0; y < GridSize; y++)
            {
                for (int x = 0; x < GridSize; x++)
                {
                    if (input[y][x] == '#') bugs.Add(new Vector3Int(x, y, 0));
                }
            }
            return bugs;
        }

        private Dictionary<Vector3Int, List<Vector3Int>> adjacentTable2 = new Dictionary<Vector3Int, List<Vector3Int>>();
        // there has to be a better way, but this works...
        // Z+ indicates moving inward a layer, Z- indicates moving out
        private List<Vector3Int> AdjacentTiles2(Vector3Int center)
        {
            if (!adjacentTable2.ContainsKey(center))
            {
                List<Vector3Int> adj = new List<Vector3Int>(8);

                // North tiles
                if (center.y == 0)
                {
                    adj.Add(new Vector3Int(2, 1, center.z - 1));
                }
                else if (center.x == 2 && center.y == 3)
                {
                    adj.Add(new Vector3Int(0, 4, center.z + 1));
                    adj.Add(new Vector3Int(1, 4, center.z + 1));
                    adj.Add(new Vector3Int(2, 4, center.z + 1));
                    adj.Add(new Vector3Int(3, 4, center.z + 1));
                    adj.Add(new Vector3Int(4, 4, center.z + 1));
                }
                else
                {
                    adj.Add(new Vector3Int(center.x, center.y - 1, center.z));
                }

                // South tiles
                if (center.y == 4)
                {
                    adj.Add(new Vector3Int(2, 3, center.z - 1));
                }
                else if (center.x == 2 && center.y == 1)
                {
                    adj.Add(new Vector3Int(0, 0, center.z + 1));
                    adj.Add(new Vector3Int(1, 0, center.z + 1));
                    adj.Add(new Vector3Int(2, 0, center.z + 1));
                    adj.Add(new Vector3Int(3, 0, center.z + 1));
                    adj.Add(new Vector3Int(4, 0, center.z + 1));
                }
                else
                {
                    adj.Add(new Vector3Int(center.x, center.y + 1, center.z));
                }

                // East tiles
                if (center.x == 4)
                {
                    adj.Add(new Vector3Int(3, 2, center.z - 1));
                }
                else if (center.x == 1 && center.y == 2)
                {
                    adj.Add(new Vector3Int(0, 0, center.z + 1));
                    adj.Add(new Vector3Int(0, 1, center.z + 1));
                    adj.Add(new Vector3Int(0, 2, center.z + 1));
                    adj.Add(new Vector3Int(0, 3, center.z + 1));
                    adj.Add(new Vector3Int(0, 4, center.z + 1));
                }
                else
                {
                    adj.Add(new Vector3Int(center.x + 1, center.y, center.z));
                }

                // West tiles
                if (center.x == 0)
                {
                    adj.Add(new Vector3Int(1, 2, center.z - 1));
                }
                else if (center.x == 3 && center.y == 2)
                {
                    adj.Add(new Vector3Int(4, 0, center.z + 1));
                    adj.Add(new Vector3Int(4, 1, center.z + 1));
                    adj.Add(new Vector3Int(4, 2, center.z + 1));
                    adj.Add(new Vector3Int(4, 3, center.z + 1));
                    adj.Add(new Vector3Int(4, 4, center.z + 1));
                }
                else
                {
                    adj.Add(new Vector3Int(center.x - 1, center.y, center.z));
                }

                adjacentTable2[center] = adj;
            }
            return adjacentTable2[center];
        }

        /*
        This was kind of fun. Run like a normal game of life but use a z axis
        to track which layer the bugs are on. Finding neighboring cells is a
        bit convoluted but it works and is the major bottleneck for the whole
        thing. But it works, and ~300ms is good enough for me.
        */
        public override object Task2()
        {
            HashSet<Vector3Int> Bugs = ParseInput2();

            HashSet<Vector3Int> changes = new HashSet<Vector3Int>();
            HashSet<int> layers = new HashSet<int>() { -1, 0, 1 };

            for (int _ = 0; _ < 200; _++)
            {
                changes.Clear();
                Vector3Int current = new Vector3Int();
                foreach (int z in layers)
                {
                    current.z = z;
                    for (int y = 0; y < input.Length; y++)
                    {
                        current.y = y;
                        for (int x = 0; x < input[0].Length; x++)
                        {
                            if (x == 2 && y == 2) continue;
                            current.x = x;

                            int adjBugs = 0;
                            foreach (Vector3Int adj in AdjacentTiles2(current))
                            {
                                if (Bugs.Contains(adj)) adjBugs++;
                            }
                            if (Bugs.Contains(current) && adjBugs != 1) changes.Add(current);
                            else if (!Bugs.Contains(current) && (adjBugs == 1 || adjBugs == 2)) changes.Add(current);
                        }
                    }
                }
                foreach (Vector3Int v in changes)
                {
                    layers.Add(v.z - 1);
                    layers.Add(v.z + 1);
                    if (Bugs.Contains(v)) Bugs.Remove(v);
                    else Bugs.Add(v);
                }
            }
            return Bugs.Count;
        }
    }
}