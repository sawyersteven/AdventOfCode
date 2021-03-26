using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2019
{
    public class Challenge20 : Challenge
    {
        private Dictionary<Vector2Int, Gate> gates;
        private Gate entry;
        private char[,] maze;
        public override object Task1()
        {
            maze = Utils.InputToCharArray(input, false);
            gates = FindGates();
            FillGateBranches();
            entry = RemoveEntryGate();

            List<(Gate, int)> pq = new List<(Gate, int)>();
            HashSet<Gate> visited = new HashSet<Gate>();
            pq.Add((entry, 0));

            while (pq.Count > 0)
            {
                pq.Sort((a, b) => b.Item2 - a.Item2);

                (Gate current, int dist) = pq[^1];
                pq.RemoveAt(pq.Count - 1);
                visited.Add(current);

                if (current == null) return dist - 1;

                foreach ((Gate g, int cost) in current.Branches)
                {
                    if (visited.Contains(g)) continue;
                    pq.Add((g, dist + cost + 1));
                }
            }
            return null;
        }

        private Gate RemoveEntryGate()
        {
            foreach (var kv in gates)
            {
                if (kv.Value.Name == "(A, A)")
                {
                    gates.Remove(kv.Key);
                    return kv.Value;
                }
            }
            throw new System.Exception();
        }

        private void FillGateBranches()
        {
            foreach (var kv in gates)
            {
                Vector2Int start = kv.Key;
                HashSet<Vector2Int> visited = new HashSet<Vector2Int>();
                Queue<(Vector2Int, int)> queue = new Queue<(Vector2Int, int)>();

                queue.Enqueue((start, 0));
                while (queue.Count > 0)
                {
                    (Vector2Int current, int dist) = queue.Dequeue();

                    visited.Add(current);

                    foreach (Vector2Int v in Vector2Int.CardinalDirections)
                    {
                        Vector2Int next = current + v;
                        if (visited.Contains(next)) continue;
                        if (gates.ContainsKey(next) && gates[next].Name != "(A, A)")
                        {
                            kv.Value.Branches.Add((gates[next].Exit, dist + 1));
                            continue;
                        }
                        if (maze[next.y, next.x] == '.') queue.Enqueue((next, dist + 1));
                    }
                }
            }
        }

        private class Gate
        {
            public readonly Vector2Int Location;
            public Gate Exit;
            public readonly bool IsExterior;
            public readonly string Name;
            // Next gate, steps to get there
            public List<(Gate, int)> Branches = new List<(Gate, int)>();

            public Gate(Vector2Int location, bool isExterior, string name)
            {
                Location = location;
                IsExterior = isExterior;
                Name = name;
            }

            public override string ToString()
            {
                return Name;
            }
        }

        // This is a convoluted mess, but the input isn't easy to worth with
        private Dictionary<Vector2Int, Gate> FindGates()
        {
            var gates = new Dictionary<Vector2Int, Gate>();

            ((char, char), Vector2Int) FindGate(int x, int y)
            {
                (char, char) k = (' ', ' ');
                Vector2Int v = Vector2Int.Zero;
                if (maze[y + 1, x] == '.')
                {
                    k = (maze[y - 1, x], maze[y, x]);
                    v = new Vector2Int(x, y + 1);
                }
                else if (maze[y - 1, x] == '.')
                {
                    k = (maze[y, x], maze[y + 1, x]);
                    v = new Vector2Int(x, y - 1);
                }
                else if (maze[y, x + 1] == '.')
                {
                    k = (maze[y, x - 1], maze[y, x]);
                    v = new Vector2Int(x + 1, y);
                }
                else if (maze[y, x - 1] == '.')
                {
                    k = (maze[y, x], maze[y, x + 1]);
                    v = new Vector2Int(x - 1, y);
                }
                return (k, v);
            }
            int h = maze.GetLength(0);
            int w = maze.GetLength(1);

            // inside gates
            for (int y = 2; y < h - 2; y++)
            {
                for (int x = 2; x < w - 2; x++)
                {
                    if (maze[y, x] >= 'A')
                    {
                        var g = FindGate(x, y);
                        if (g.Item2 == Vector2Int.Zero) continue;
                        gates[g.Item2] = new Gate(g.Item2, true, g.Item1.ToString());
                    }
                }
            }

            // left/right outside
            for (int y = 2; y < h - 2; y++)
            {
                if (maze[y, 1] >= 'A')
                {
                    var g = FindGate(1, y);
                    if (g.Item2 != Vector2Int.Zero)
                    {
                        gates[g.Item2] = new Gate(g.Item2, false, g.Item1.ToString());
                    }
                }
                if (maze[y, w - 2] >= 'A')
                {
                    var g = FindGate(w - 2, y);
                    if (g.Item2 != Vector2Int.Zero)
                    {
                        gates[g.Item2] = new Gate(g.Item2, false, g.Item1.ToString());
                    }
                }
            }

            // top/bottom outside
            for (int x = 2; x < w - 2; x++)
            {
                if (maze[1, x] >= 'A')
                {
                    var g = FindGate(x, 1);
                    if (g.Item2 != Vector2Int.Zero)
                    {
                        gates[g.Item2] = new Gate(g.Item2, false, g.Item1.ToString());
                    }
                }
                if (maze[h - 2, x] >= 'A')
                {
                    var g = FindGate(x, h - 2);
                    if (g.Item2 != Vector2Int.Zero)
                    {
                        gates[g.Item2] = new Gate(g.Item2, false, g.Item1.ToString());
                    }
                }
            }

            foreach (var kv in gates)
            {
                foreach (var kv2 in gates)
                {
                    if (kv.Key == kv2.Key) continue;
                    if (kv.Value.Name == kv2.Value.Name)
                    {
                        kv.Value.Exit = kv2.Value;
                    }
                }
            }

            return gates;
        }

        private int SortT2Nodes(T2Node a, T2Node b)
        {
            if (b.Depth == a.Depth) return b.TotalDist - a.TotalDist;
            return b.Depth - a.Depth;
        }

        private class T2Node
        {
            public Gate Location;
            public int TotalDist;
            public int Depth;
        }

        /*
            Took a while for me to realize that keeping track of visited nodes
            isn't going to work, even if they are paired with the recursion
            level of the maze. I believe the key here is sorting the pseudo-
            priority queue by Depth first, then TotalDistance. The queue
            doesn't even get terribly populated over time and is fast.

            I didn't look at any other solutions or descriptions for this
            and I'll admit to being a bit proud of this solution.
        */
        public override object Task2()
        {
            List<T2Node> pq = new List<T2Node>();

            T2Node startNode = new T2Node();
            startNode.Location = entry;
            pq.Add(startNode);

            while (pq.Count > 0)
            {
                pq.Sort(SortT2Nodes);

                T2Node current = pq[^1];
                pq.RemoveAt(pq.Count - 1);

                if (current.Location == null && current.Depth == -1) return current.TotalDist - 1;

                foreach ((Gate g, int cost) in current.Location.Branches)
                {

                    if (g == null)
                    {
                        if (current.Depth != 0) continue;
                        T2Node n = new T2Node() { Location = g, TotalDist = current.TotalDist + cost + 1, Depth = current.Depth - 1 };
                        pq.Add(n);
                        continue;
                    }

                    if (current.Depth == 0 && g.IsExterior) continue;
                    T2Node next = new T2Node() { Location = g, TotalDist = current.TotalDist + cost + 1, Depth = current.Depth + (g.IsExterior ? -1 : 1) };
                    pq.Add(next);
                }
            }
            return null;
        }
    }
}