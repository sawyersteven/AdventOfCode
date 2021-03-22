using AdventOfCode;
using System;
using System.Collections.Generic;
using Grids;

namespace Advent2018
{
    public class Challenge22 : Challenge
    {
        private const char wet = '=';
        private const char rocky = '.';
        private const char narrow = '|';

        private const int gridPadding = 75;

        private char[] types = new char[] { rocky, wet, narrow };

        private int[,] MakeErosionMap()
        {
            const int erosionMod = 20183;
            int[,] map = new int[targetCoords.y + gridPadding, targetCoords.x + gridPadding];
            int w = map.GetLength(1);
            int h = map.GetLength(0);

            map[0, 0] = depth % 20183;
            for (int y = 1; y < h; y++)
            {
                map[y, 0] = ((y * 48271) + depth) % erosionMod;
            }
            for (int x = 1; x < w; x++)
            {
                map[0, x] = ((x * 16807) + depth) % erosionMod;
            }

            for (int y = 1; y < h; y++)
            {
                for (int x = 1; x < w; x++)
                {
                    if (x == targetCoords.x && y == targetCoords.y)
                    {
                        map[y, x] = map[0, 0];
                        continue;
                    }
                    map[y, x] = ((map[y, x - 1] * map[y - 1, x]) + depth) % erosionMod;
                }
            }
            return map;
        }

        private char[,] MakeTerrainMap(int[,] geoMap)
        {
            int w = geoMap.GetLength(1);
            int h = geoMap.GetLength(0);
            char[,] map = new char[h, w];

            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    map[y, x] = types[geoMap[y, x] % 3];
                }
            }
            return map;
        }

        int depth;
        Vector2Int targetCoords;
        char[,] map;
        public override object Task1()
        {
            depth = int.Parse(input[0].Split(' ')[1]); ;
            int[] t = Array.ConvertAll(input[1].Split(' ')[1].Split(','), int.Parse);
            targetCoords = new Vector2Int(t[0], t[1]);
            map = MakeTerrainMap(MakeErosionMap());

            int risk = 0;
            for (int y = 0; y < targetCoords.y + 1; y++)
            {
                for (int x = 0; x < targetCoords.x + 1; x++)
                {
                    switch (map[y, x])
                    {
                        case wet:
                            risk += 1;
                            break;
                        case narrow:
                            risk += 2;
                            break;
                    }
                }
            }
            return risk;
        }

        private class Location
        {
            public int Tool;
            public Vector2Int Point;

            public override string ToString()
            {
                return $"{Point}, {Tool}";
            }

            public override int GetHashCode()
            {
                return (Point.y * 10000) + (Point.x * 10) + Tool;
            }

            public override bool Equals(object obj)
            {
                Location other = obj as Location;
                if (other is null) return false;
                return GetHashCode() == other.GetHashCode();
            }

            public static bool operator ==(Location a, Location b)
            {
                return a.GetHashCode() == b.GetHashCode();
            }

            public static bool operator !=(Location a, Location b)
            {
                return !(a == b);
            }
        }

        private const int none = 0;
        private const int torch = 1;
        private const int gear = 2;

        private Dictionary<char, (int, int)> CompatibleTools = new Dictionary<char, (int, int)>()
        {
            {rocky, (gear, torch)},
            {narrow, (none, torch)},
            {wet, (none, gear)}
        };

        public override object Task2()
        {
            // Basically A* but with dictionaries to hold the open/closed set
            // as <LocationData, Score> and the heuristic is just step + tool cost
            int maxY = map.GetLength(0) - 1;
            int maxX = map.GetLength(1) - 1;

            Dictionary<Location, int> open = new Dictionary<Location, int>();
            Dictionary<Location, int> closed = new Dictionary<Location, int>();

            Location start = new Location() { Tool = torch, Point = new Vector2Int(0, 0) };
            closed[start] = 0;
            open.Add(start, 0);

            Location target = new Location() { Point = targetCoords, Tool = torch };

            while (open.Count > 0)
            {
                (Location coord, int time) = MinScore(open);

                open.Remove(coord);

                if (time > closed[coord]) continue;

                if (coord == target) return closed[coord];

                // create location for this point with other tool
                (int, int) compatTools = CompatibleTools[map[coord.Point.y, coord.Point.x]];
                int otherTool = -1;
                otherTool = (compatTools.Item1 == coord.Tool) ? compatTools.Item2 : compatTools.Item1;

                Location altL = new Location() { Point = coord.Point, Tool = otherTool };

                int altScore = closed[coord] + 7;
                if (!closed.ContainsKey(altL) || closed[altL] > altScore)
                {
                    closed[altL] = altScore;
                    open[altL] = altScore;
                }

                foreach (Vector2Int otherP in AdjacentMoves(coord.Point, maxX, maxY))
                {
                    (int, int) otherCompatTools = CompatibleTools[map[otherP.y, otherP.x]];
                    if (otherCompatTools.Item1 != coord.Tool && otherCompatTools.Item2 != coord.Tool) continue;

                    Location nextMove = new Location() { Point = otherP, Tool = coord.Tool };
                    int score = closed[coord] + 1;
                    if (!closed.ContainsKey(nextMove) || closed[nextMove] > score)
                    {
                        closed[nextMove] = score;
                        open[nextMove] = score;
                    }
                }
            }

            return null;
        }

        private List<Vector2Int> AdjacentMoves(Vector2Int point, int maxX, int maxY)
        {
            List<Vector2Int> adjacents = new List<Vector2Int>();
            if (point.y < maxY) adjacents.Add(new Vector2Int(point.x, point.y + 1));
            if (point.y > 0) adjacents.Add(new Vector2Int(point.x, point.y - 1));
            if (point.x < maxX) adjacents.Add(new Vector2Int(point.x + 1, point.y));
            if (point.x > 0) adjacents.Add(new Vector2Int(point.x - 1, point.y));
            return adjacents;
        }

        private KeyValuePair<Location, int> MinScore(Dictionary<Location, int> openSet)
        {
            KeyValuePair<Location, int> low = new KeyValuePair<Location, int>(null, int.MaxValue);
            foreach (var kv in openSet)
            {
                if (kv.Value < low.Value)
                {
                    low = kv;
                }
            }
            return low;
        }
    }
}