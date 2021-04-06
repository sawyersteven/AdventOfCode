using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2017
{
    public class Challenge14 : Challenge
    {
        byte[][] DenseHashes = new byte[128][];
        public override object Task1()
        {
            int totalBits = 0;
            for (int i = 0; i < 128; i++)
            {
                byte[] denseHash = new Challenge10.KnotHash($"{rawInput}-{i}").DenseHash;
                DenseHashes[i] = denseHash;
                foreach (byte b in denseHash)
                {
                    totalBits += BitsInByte(b);
                }
            }
            return totalBits;
        }

        private int BitsInByte(byte b)
        {
            int bits = 0;
            while (b != 0)
            {
                bits += b & 1;
                b >>= 1;
            }
            return bits;
        }

        public override object Task2()
        {
            bool[,] grid = new bool[128, 128];

            for (int y = 0; y < 128; y++)
            {
                byte[] rowBytes = DenseHashes[y];
                int x = 0;
                for (int i = 0; i < rowBytes.Length; i++)
                {
                    byte b = rowBytes[i];
                    for (int _ = 0; _ < 8; _++)
                    {
                        grid[y, x] = (b & 0b1000_0000) != 0;
                        b <<= 1;
                        x++;
                    }
                }
            }

            int regionCount = 0;
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>();
            Vector2Int start = new Vector2Int();
            for (; start.y < 128; start.y++)
            {
                for (start.x = 0; start.x < 128; start.x++)
                {
                    if (visited.Contains(start) || grid[start.y, start.x] == false) continue;
                    visited.UnionWith(FindRegion(grid, start));
                    regionCount++;
                }
            }
            return regionCount;
        }

        private HashSet<Vector2Int> FindRegion(bool[,] grid, Vector2Int start)
        {
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>();
            Queue<Vector2Int> q = new Queue<Vector2Int>();
            q.Enqueue(start);

            while (q.Count > 0)
            {
                Vector2Int current = q.Dequeue();
                visited.Add(current);

                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector2Int next = current + v;
                    if (next.x < 0 || next.y < 0 || next.x > 127 || next.y > 127) continue;
                    if (visited.Contains(next)) continue;
                    if (grid[next.y, next.x] == false) continue;
                    q.Enqueue(next);
                }
            }
            return visited;
        }
    }
}
