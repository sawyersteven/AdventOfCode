using AdventOfCode;
using System.Collections.Generic;
using Grids;

namespace Advent2018
{
    public class Challenge20 : Challenge
    {
        Dictionary<char, Vector2Int> dirs = new Dictionary<char, Vector2Int>()
        {{'N', Vector2Int.Down},
         {'E', Vector2Int.Right},
         {'S', Vector2Int.Up},
         {'W', Vector2Int.Left}
        };

        Dictionary<Vector2Int, int> distances = new Dictionary<Vector2Int, int>();
        public override object Task1()
        {
            int dist = 0;
            Stack<(int, Vector2Int)> forks = new Stack<(int, Vector2Int)>();
            Vector2Int position = new Vector2Int();
            foreach (char c in input[0].Substring(1, input[0].Length - 2))
            {
                switch (c)
                {
                    case '(':
                        forks.Push((dist, position));
                        break;
                    case ')':
                        (dist, position) = forks.Pop();
                        break;
                    case '|':
                        (dist, position) = forks.Peek();
                        break;
                    default:
                        dist++;
                        position += dirs[c];

                        if (!distances.ContainsKey(position))
                        {
                            distances[position] = dist;
                        }
                        else if (dist < distances[position])
                        {
                            distances[position] = dist;
                        }
                        break;
                }
            }

            int maxDist = 0;
            foreach (int v in distances.Values)
            {
                if (v > maxDist) maxDist = v;
            }

            return maxDist;
        }

        public override object Task2()
        {
            int counter = 0;
            foreach (int v in distances.Values)
            {
                if (v >= 1000) counter++;
            }

            return counter;
        }
    }
}