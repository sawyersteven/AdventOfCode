using System.Collections.Generic;
using System;

namespace Grids
{
    public class AStar
    {
        public class Node
        {
            public Node Parent;
            public Vector2Int Position;
            public int FScore => Steps + HeuristicScore;
            public int Steps;
            public int HeuristicScore;

            public Node(Vector2Int position, Node parent)
            {
                Position = position;
                Parent = parent;
            }

            public override int GetHashCode()
            {
                return base.GetHashCode();
            }

            public override string ToString()
            {
                return Position.ToString();
            }
        }

        private readonly Func<Vector2Int, Vector2Int, int> Heuristic;

        public AStar(Func<Vector2Int, Vector2Int, int> heuristic = null)
        {

            Heuristic = heuristic ?? ManhattanDistance;
        }

        private int ManhattanDistance(Vector2Int a, Vector2Int b) => a.ManhattanDistance(b);


        private IEnumerable<Vector2Int> NextMoves(Vector2Int position, HashSet<Vector2Int> blockedCells)
        {
            foreach (Vector2Int cd in Vector2Int.CardinalDirections)
            {
                Vector2Int n = position + cd;
                if (!blockedCells.Contains(n)) yield return n;

            }
        }
        public Node[] FindPath(Vector2Int start, Vector2Int end, HashSet<Vector2Int> blockedCells)
        {
            if (start == end) return new Node[0];

            Node startNode = new Node(start, null);
            List<Node> open = new List<Node>();
            HashSet<Vector2Int> closed = new HashSet<Vector2Int>(); // hashset

            open.Add(startNode);

            Node current;
            while (open.Count > 0)
            {
                open.Sort((a, b) => (a.FScore >= b.FScore) ? 1 : -1); // actually faster than a sorted set.
                current = open[0];
                open.RemoveAt(0);
                closed.Add(current.Position);

                if (current.Position == end)
                {
                    return MakePath(current);
                }

                foreach (Vector2Int v in NextMoves(current.Position, blockedCells))
                {
                    if (closed.Contains(v)) continue;

                    Node n = new Node(v, current);
                    n.Steps = current.Steps + 1;
                    n.HeuristicScore = Heuristic(n.Position, end);

                    Node existing = MatchingPoint(open, v);
                    if (existing == null)
                    {
                        open.Add(n);
                    }
                    else
                    {
                        if (n.FScore < existing.FScore)
                        {
                            open.Remove(existing);
                            open.Add(n);
                        }
                    }
                }
            }
            return null;
        }

        private Node[] MakePath(Node endNode)
        {
            Node[] path = new Node[endNode.Steps];
            for (int i = path.Length - 1; i > -1; i--)
            {
                path[i] = endNode;
                endNode = endNode.Parent;
            }
            return path;
        }

        private Node MatchingPoint(IEnumerable<Node> nodes, Vector2Int position)
        {
            foreach (Node n in nodes)
            {
                if (n.Position == position) return n;
            }
            return null;
        }
    }
}