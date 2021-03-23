using System;
using System.Collections.Generic;

namespace Grids
{

    public static class BFS
    {
        public static Vector2Int[] FindTarget(Vector2Int startLocation, Vector2Int target, HashSet<Vector2Int> blockedCells)
        {
            return FindTarget(startLocation, new HashSet<Vector2Int>() { target }, blockedCells);
        }

        // Searches until any target is found
        public static Vector2Int[] FindTarget(Vector2Int startLocation, HashSet<Vector2Int> targets, HashSet<Vector2Int> blockedCells)
        {
            Queue<Vector2Int[]> queue = new Queue<Vector2Int[]>();
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>(blockedCells);

            queue.Enqueue(new Vector2Int[] { startLocation });

            while (queue.Count > 0)
            {
                Vector2Int[] path = queue.Dequeue();
                Vector2Int endNode = path[^1];

                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector2Int nextStep = endNode + v;
                    if (visited.Contains(nextStep)) continue;

                    Vector2Int[] np = new Vector2Int[path.Length + 1];
                    Array.Copy(path, np, path.Length);
                    np[^1] = nextStep;
                    if (targets.Contains(nextStep)) return np;
                    queue.Enqueue(np);
                    visited.Add(nextStep);
                }
            }
            return null;
        }
    }
}