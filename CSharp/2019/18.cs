using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2019
{
    public class Challenge18 : Challenge
    {
        private class GraphNode
        {
            public int Steps;
            public HashSet<Vector2Int> UnlockedDoors = new HashSet<Vector2Int>();
            public Vector2Int Location;
            public GraphNode(int steps, Vector2Int location)
            {
                Location = location;
                Steps = steps;
            }
        }

        private Dictionary<Vector2Int, int> PathsToAllKeys(Vector2Int start, int keyMask)
        {
            // End position, steps
            Dictionary<Vector2Int, int> result = new Dictionary<Vector2Int, int>();

            Queue<(Vector2Int, int)> queue = new Queue<(Vector2Int, int)>();
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>(walls);
            visited.Add(start);

            queue.Enqueue((start, 0));
            int foundKeys = 0;
            while (queue.Count > 0)
            {
                (Vector2Int endNode, int currentLen) = queue.Dequeue();

                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector2Int nextStep = endNode + v;
                    if (visited.Contains(nextStep)) continue;
                    visited.Add(nextStep);

                    if (doors.TryGetValue(nextStep, out int doorID) && (keyMask & doorID) == doorID) continue;

                    if (keys.TryGetValue(nextStep, out int keyID) && (keyMask & keyID) == keyID)
                    {
                        result[nextStep] = currentLen + 1;
                        foundKeys |= keyID;
                        if (foundKeys == keyMask)
                        {
                            queue.Clear();
                            break;
                        }
                        continue;
                    }

                    queue.Enqueue((nextStep, currentLen + 1));
                }
            }
            return result;
        }

        private Dictionary<(Vector2Int, int), int> CachedTrips = new Dictionary<(Vector2Int, int), int>();
        private int GetDistance(Vector2Int currentLocation, int keyMask)
        {
            if (keyMask == 0)
            {
                return 0;
            }
            if (CachedTrips.TryGetValue((currentLocation, keyMask), out int dist))
            {
                return dist;
            }

            int best = int.MaxValue;
            foreach (var kv in PathsToAllKeys(currentLocation, keyMask))
            {
                int remainingKeys = keyMask;
                remainingKeys &= ~keys[kv.Key];
                int d = kv.Value + GetDistance(kv.Key, remainingKeys);
                if (d < best) best = d;
            }

            CachedTrips[(currentLocation, keyMask)] = best;
            return best;
        }

        HashSet<Vector2Int> walls;
        Dictionary<Vector2Int, int> keys;
        Dictionary<Vector2Int, int> doors;
        Vector2Int startLoc;

        private void ParseInput()
        {
            doors = new Dictionary<Vector2Int, int>();
            keys = new Dictionary<Vector2Int, int>();

            walls = new HashSet<Vector2Int>();

            startLoc = new Vector2Int();

            for (int y = 0; y < input.Length; y++)
            {
                for (int x = 0; x < input[0].Length; x++)
                {
                    char c = input[y][x];
                    switch (c)
                    {
                        case '#':
                            walls.Add(new Vector2Int(x, y));
                            break;
                        case '@':
                            startLoc = new Vector2Int(x, y);
                            break;
                        case char i when (i >= 'A' && i <= 'Z'):
                            doors[new Vector2Int(x, y)] = 1 << (i + 32);
                            break;
                        case char i when (i >= 'a' && i <= 'z'):
                            keys[new Vector2Int(x, y)] = 1 << i;
                            break;
                    }
                }
            }

            for (int y = 1; y < input.Length - 1; y++)
            {
                for (int x = 1; x < input[0].Length - 1; x++)
                {
                    char c = input[y][x];
                    if (c == '.')
                    {
                        int wallCount = 0;
                        if (input[y + 1][x] == '#') wallCount++;
                        if (input[y - 1][x] == '#') wallCount++;
                        if (input[y][x + 1] == '#') wallCount++;
                        if (input[y][x - 1] == '#') wallCount++;
                        if (wallCount > 3)
                        {
                            walls.Add(new Vector2Int(x, y));
                        }
                    }
                }
            }
        }

        public override object Task1()
        {
            ParseInput();

            int keyMask = 0;
            foreach (int i in keys.Values)
            {
                keyMask += i;
            }

            return GetDistance(startLoc, keyMask);
        }

        public override object Task2()
        {

            return null;
        }
    }
}
