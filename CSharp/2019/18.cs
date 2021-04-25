using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2019
{
    public class Challenge18 : Challenge
    {

        HashSet<Vector2Int> walls;
        Dictionary<Vector2Int, int> doors;
        Dictionary<Vector2Int, int> keys;
        int allKeyMask;
        Vector3Int T1Start;

        private void MakeMap()
        {
            walls = new HashSet<Vector2Int>();
            doors = new Dictionary<Vector2Int, int>();
            keys = new Dictionary<Vector2Int, int>();

            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input.Length; x++)
                {
                    char c = input[y][x];
                    switch (c)
                    {
                        case '@':
                            T1Start = new Vector3Int(x, y, 0);
                            break;
                        case '#':
                            walls.Add(new Vector2Int(x, y));
                            break;
                        case '.':
                            break;
                        default:
                            int k = 1;
                            if (c < 'a')
                            {
                                k <<= (c - 'A');
                                doors[new Vector2Int(x, y)] = k;
                            }
                            else
                            {
                                k <<= (c - 'a');
                                keys[new Vector2Int(x, y)] = k;
                            }
                            allKeyMask |= k;
                            break;
                    }
                }
            }
        }

        private int Search(Vector3Int start, int ownedKeys)
        {
            Queue<(Vector3Int, int)> q = new Queue<(Vector3Int, int)>();
            HashSet<Vector3Int> visited = new HashSet<Vector3Int>();
            var init = (start, 0);
            init.Item1.z = ownedKeys;
            q.Enqueue(init);

            while (q.Count > 0)
            {
                (Vector3Int current, int steps) = q.Dequeue();
                if ((current.z & allKeyMask) == allKeyMask) return steps;


                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector3Int next = current + v;
                    if (walls.Contains(next) || visited.Contains(next)) continue;
                    if (doors.TryGetValue(next, out int d) && (next.z & d) != d) continue;

                    visited.Add(next);

                    if (keys.TryGetValue(next, out int k)) next.z |= k;

                    q.Enqueue((next, steps + 1));
                }
            }

            return -1;
        }

        public override object Task1()
        {
            MakeMap();
            return Search(T1Start, 0);
        }

        public override object Task2()
        {
            foreach (Vector2Int v in Vector2Int.CardinalDirections)
            {
                walls.Add(T1Start + v);
            }

            // top left
            int total = 0;
            int ownedKeys = allKeyMask;
            Vector2Int scan = Vector2Int.Zero;
            for (scan.y = 0; scan.y < T1Start.y; scan.y++)
            {
                for (scan.x = 0; scan.x < T1Start.x; scan.x++)
                {
                    if (keys.TryGetValue(scan, out int k)) ownedKeys ^= k;
                }
            }

            Vector3Int start = T1Start;
            start.x--;
            start.y--;
            total += Search(start, ownedKeys);

            // top right
            ownedKeys = allKeyMask;
            for (scan.y = 0; scan.y < T1Start.y; scan.y++)
            {
                for (scan.x = T1Start.x + 1; scan.x < T1Start.x * 2; scan.x++)
                {
                    if (keys.TryGetValue(scan, out int k)) ownedKeys ^= k;
                }
            }

            start = T1Start;
            start.x++;
            start.y--;
            total += Search(start, ownedKeys);

            // bottom right
            ownedKeys = allKeyMask;
            for (scan.y = T1Start.x + 1; scan.y < T1Start.y * 2; scan.y++)
            {
                for (scan.x = T1Start.x + 1; scan.x < T1Start.x * 2; scan.x++)
                {
                    if (keys.TryGetValue(scan, out int k)) ownedKeys ^= k;
                }
            }

            start = T1Start;
            start.x++;
            start.y++;
            total += Search(start, ownedKeys);

            // bottom left
            ownedKeys = allKeyMask;
            for (scan.y = T1Start.x + 1; scan.y < T1Start.y * 2; scan.y++)
            {
                for (scan.x = 0; scan.x < T1Start.x; scan.x++)
                {
                    if (keys.TryGetValue(scan, out int k)) ownedKeys ^= k;
                }
            }

            start = T1Start;
            start.x--;
            start.y++;
            total += Search(start, ownedKeys);

            return total;
        }
    }
}
