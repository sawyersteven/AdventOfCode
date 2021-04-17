using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2016
{
    public class Challenge01 : Challenge
    {
        public override object Task1()
        {
            int direction = 2;
            Vector2Int location = Vector2Int.Zero;
            foreach (string s in rawInput.Split(", "))
            {
                direction += s[0] == 'R' ? 1 : 3;
                direction %= 4;
                location += Vector2Int.CardinalDirections[direction] * int.Parse(s.Substring(1));
            }
            return location.ManhattanDistance(Vector2Int.Zero);
        }

        public override object Task2()
        {
            HashSet<Vector2Int> history = new HashSet<Vector2Int>();

            int direction = 2;
            Vector2Int location = Vector2Int.Zero;
            foreach (string s in rawInput.Split(", "))
            {
                direction += s[0] == 'R' ? 1 : 3;
                direction %= 4;
                int dist = int.Parse(s.Substring(1));
                for (int i = 0; i < dist; i++)
                {
                    location += Vector2Int.CardinalDirections[direction];
                    if (history.Contains(location))
                    {
                        return location.ManhattanDistance(Vector2Int.Zero);
                    }
                    history.Add(location);
                }
            }
            return null;
        }
    }
}
