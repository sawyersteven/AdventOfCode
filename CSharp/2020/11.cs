using AdventOfCode;
using System;
using System.Collections.Generic;
using Grids;

namespace Advent2020
{
    public class Challenge11 : Challenge
    {
        const char floor = '.';
        const char seatEmpty = 'L';
        const char seatOccupied = '#';

        private int emptySeatThreshold = 4;

        private Vector2Int[,][] adjacentCache;

        public override object Task1()
        {
            char[][] map = new char[input.Length][];
            char[][] changes = new char[input.Length][];
            for (int row = 0; row < input.Length; row++)
            {
                map[row] = input[row].ToCharArray();
                changes[row] = new char[input[row].Length];
            }

            if (adjacentCache == null)
            {
                FillAdjacentCacheT1(map);
            }

            while (true)
            {
                int c = 0;
                for (int row = 0; row < map.Length; row++)
                {
                    for (int column = 0; column < map[0].Length; column++)
                    {
                        switch (map[row][column])
                        {
                            case seatEmpty:
                                if (CanFillSeat(row, column, map))
                                {
                                    c++;
                                    changes[row][column] = seatOccupied;
                                }
                                break;
                            case seatOccupied:
                                if (CanEmptySeat(row, column, map))
                                {
                                    c++;
                                    changes[row][column] = seatEmpty;
                                }
                                break;
                        }
                    }
                }

                if (c == 0) break;
                c = 0;

                for (int row = 0; row < changes.Length; row++)
                {
                    for (int column = 0; column < changes[0].Length; column++)
                    {
                        if (changes[row][column] != '\0') map[row][column] = changes[row][column];
                    }
                    Array.Clear(changes[row], 0, changes[row].Length);
                }

            }

            int count = 0;
            foreach (char[] row in map)
            {
                foreach (char c in row)
                {
                    if (c == seatOccupied) count++;
                }
            }

            return count;
        }

        private void FillAdjacentCacheT1(char[][] map)
        {
            adjacentCache = new Vector2Int[input.Length, input[0].Length][];
            for (int row = 0; row < input.Length; row++)
            {
                for (int col = 0; col < input[row].Length; col++)
                {
                    List<Vector2Int> ch = new List<Vector2Int>();
                    for (int r = row - 1; r <= row + 1; r++)
                    {
                        if (r < 0) continue;
                        if (r > map.Length - 1) continue;
                        for (int c = col - 1; c <= col + 1; c++)
                        {
                            if (c < 0) continue;
                            if (c > map[0].Length - 1) continue;
                            if (r == row && c == col) continue;
                            ch.Add(new Vector2Int(r, c));
                        }
                    }
                    adjacentCache[row, col] = ch.ToArray();
                }
            }
        }

        private void FillAdjacentCacheT2(char[][] map)
        {
            // the x and y are backward and I can't be bothered to fix it
            Vector2Int n = new Vector2Int(-1, 0);
            Vector2Int s = new Vector2Int(1, 0);
            Vector2Int e = new Vector2Int(0, 1);
            Vector2Int w = new Vector2Int(0, -1);

            Vector2Int ne = new Vector2Int(-1, 1);
            Vector2Int nw = new Vector2Int(-1, -1);
            Vector2Int se = new Vector2Int(1, 1);
            Vector2Int sw = new Vector2Int(1, -1);

            Vector2Int[] dirs = new Vector2Int[] { n, s, e, w, ne, nw, se, sw };

            adjacentCache = new Vector2Int[input.Length, input[0].Length][];
            for (int row = 0; row < input.Length; row++)
            {
                for (int col = 0; col < input[row].Length; col++)
                {
                    List<Vector2Int> ch = new List<Vector2Int>();
                    foreach (Vector2Int direction in dirs)
                    {
                        int dist = 1;
                        Vector2Int next = direction * dist;
                        next.x += row;
                        next.y += col;
                        while (true)
                        {
                            if (next.x < 0 || next.x >= map.Length || next.y < 0 || next.y >= map[0].Length) break;

                            if (map[next.x][next.y] != floor)
                            {
                                ch.Add(next);
                                break;
                            }
                            dist++;
                            next = direction * dist;
                            next.x += row;
                            next.y += col;
                        }

                    }
                    adjacentCache[row, col] = ch.ToArray();
                }
            }
        }

        private IEnumerable<char> AdjacentSeats(int row, int column, char[][] map)
        {
            foreach (Vector2Int rc in adjacentCache[row, column])
            {
                yield return map[rc.x][rc.y];
            }
        }

        private bool CanFillSeat(int row, int column, char[][] map)
        {
            foreach (char s in AdjacentSeats(row, column, map))
            {
                if (s == seatOccupied) return false;
            }
            return true;
        }

        private bool CanEmptySeat(int row, int column, char[][] map)
        {
            int count = 0;
            foreach (char s in AdjacentSeats(row, column, map))
            {
                if (s == seatOccupied) count++;

            }
            return count >= emptySeatThreshold;
        }

        public override object Task2()
        {
            emptySeatThreshold = 5;
            char[][] map = new char[input.Length][];
            for (int row = 0; row < input.Length; row++)
            {
                map[row] = input[row].ToCharArray();
            }
            FillAdjacentCacheT2(map);
            return Task1();
        }
    }
}