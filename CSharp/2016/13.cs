using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2016
{
    public class Challenge13 : Challenge
    {
        Vector2Int target = new Vector2Int(31, 39);
        Vector3Int start = new Vector3Int(1, 1, 0);
        const int MagicNum = 1350;
        const byte Wall = 1;
        const byte Empty = 0;

        public override object Task1()
        {
            int h = target.y * 2;
            int w = target.x * 2;
            byte[,] grid = MakeGrid(w, h);

            Queue<Vector3Int> q = new Queue<Vector3Int>();
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>() { start };
            q.Enqueue(start);

            while (q.Count > 0)
            {
                Vector3Int current = q.Dequeue();
                if (current.x == target.x && current.y == target.y) return current.z;

                foreach (Vector2Int step in Vector2Int.CardinalDirections)
                {
                    Vector3Int next = current + step;
                    if (visited.Contains(next) || next.x < 0 || next.y < 0 || next.x >= w || next.y >= h) continue;
                    if (grid[next.y, next.x] == Wall) continue;

                    next.z++;
                    visited.Add(next);
                    q.Enqueue(next);
                }

            }

            return null;
        }

        private byte[,] MakeGrid(int w, int h)
        {
            byte[,] g = new byte[h, w];
            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    g[y, x] = IsWall(x, y) ? Wall : Empty;
                }
            }

            return g;
        }

        private bool IsWall(int x, int y)
        {
            long z = x * x + 3 * x + 2 * x * y + y + y * y + MagicNum;
            long bitCount = 0;

            while (z != 0)
            {
                bitCount += z & 1;
                z >>= 1;
            }
            return bitCount % 2 == 1;
        }



        public override object Task2()
        {
            int stepLimit = 50;
            int tileCount = 0;
            int h = target.y * 2;
            int w = target.x * 2;
            byte[,] grid = MakeGrid(w, h);

            Queue<Vector3Int> q = new Queue<Vector3Int>();
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>() { start };
            q.Enqueue(start);
            tileCount++;

            while (q.Count > 0)
            {
                Vector3Int current = q.Dequeue();
                if (current.z == stepLimit) continue;

                foreach (Vector2Int step in Vector2Int.CardinalDirections)
                {
                    Vector3Int next = current + step;
                    if (visited.Contains(next) || next.x < 0 || next.y < 0 || next.x >= w || next.y >= h) continue;
                    if (grid[next.y, next.x] == Wall) continue;

                    next.z++;
                    visited.Add(next);
                    q.Enqueue(next);
                    tileCount++;
                }

            }

            return tileCount;
        }
    }
}
