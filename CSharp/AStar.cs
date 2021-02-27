//#define VISUAL

using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class AStar
    {
        private class Location
        {
            public Vector2Int Point;
            public int G;
            public int H;
            public int F;
            public Location Parent;
        }

        public static Vector2Int[] FindPath(char[,] grid, Vector2Int begin, Vector2Int end, char walls)
        {
            Location start = new Location() { Point = begin };
            Location target = new Location() { Point = end };

            List<Location> Open = new List<Location>();
            List<Location> Closed = new List<Location>();
            int g = 0;

            Open.Add(start);
            Location current = null;
            while (Open.Count > 0)
            {
                current = LowestF(Open);
                Open.Remove(current);
                Closed.Add(current);

                if (current.Point == end) break;

                g++;
                var a = AdjacentMoves(grid, current.Point, walls);
                foreach (Vector2Int adjPoint in a)
                {
                    if (ContainsMatchingPoint(Closed, adjPoint)) continue;

                    Location adjacent = new Location() { Point = adjPoint };

                    if (!ContainsMatchingPoint(Open, adjacent.Point))
                    {
                        adjacent.G = g;
                        adjacent.H = HScore(adjacent.Point, target.Point);
                        adjacent.F = adjacent.G + adjacent.H;
                        adjacent.Parent = current;
                        Open.Insert(0, adjacent);
                    }
                    else
                    {
                        if (g + adjacent.H < adjacent.F)
                        {
                            adjacent.G = g;
                            adjacent.F = adjacent.G + adjacent.H;
                            adjacent.Parent = current;
                        }
                    }
                }
#if VISUAL
                Console.ForegroundColor = ConsoleColor.DarkGray;
                Console.SetCursorPosition(current.Point.x, current.Point.y + 4);
                Console.Write('·');
                System.Threading.Thread.Sleep(50);
#endif
            }
#if VISUAL
            Console.ForegroundColor = ConsoleColor.Red;
            while (current.Parent != null)
            {
                Console.SetCursorPosition(current.Point.x, current.Point.y + 4);
                Console.Write('■');
                current = current.Parent;
                System.Threading.Thread.Sleep(50);
            }
            Console.ResetColor();
            Console.SetCursorPosition(grid.GetLength(1) + 10, grid.GetLength(0));
#endif

            int len = 0;
            Location n = current;
            while (n.Parent != null)
            {
                len += 1;
                n = n.Parent;
            }
            Vector2Int[] Path = new Vector2Int[len];
            for (int i = len - 1; i >= 0; i--)
            {
                Path[i] = current.Point;
                current = current.Parent;
            }
            return Path;
        }

        static int HScore(Vector2Int location, Vector2Int target)
        {
            return Math.Abs(target.x - location.x) + Math.Abs(target.y - location.y);
        }

        private static bool ContainsMatchingPoint(List<Location> locations, Vector2Int point)
        {
            for (int i = 0; i < locations.Count; i++)
            {
                if (locations[i].Point == point) return true;
            }
            return false;
        }

        private static List<Vector2Int> AdjacentMoves(char[,] grid, Vector2Int point, char walls) // can return V2?
        {
            List<Vector2Int> adjacents = new List<Vector2Int>();
            if (grid[point.y + 1, point.x] != walls) adjacents.Add(new Vector2Int(point.x, point.y + 1));
            if (grid[point.y - 1, point.x] != walls) adjacents.Add(new Vector2Int(point.x, point.y - 1));
            if (grid[point.y, point.x + 1] != walls) adjacents.Add(new Vector2Int(point.x + 1, point.y));
            if (grid[point.y, point.x - 1] != walls) adjacents.Add(new Vector2Int(point.x - 1, point.y));
            return adjacents;
        }

        private static Location LowestF(List<Location> locations)
        {
            Location lowest = locations[0];
            for (int i = 1; i < locations.Count; i++)
            {
                if (locations[i].F < lowest.F) lowest = locations[i];
            }
            return lowest;
        }
    }
}