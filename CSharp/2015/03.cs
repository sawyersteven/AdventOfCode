using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2015
{
    public class Challenge03 : Challenge
    {
        public override object Task1()
        {
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>();
            Vector2Int start = Vector2Int.Zero;

            foreach (char c in rawInput)
            {
                switch (c)
                {
                    case '^':
                        start += Vector2Int.Up;
                        break;
                    case 'v':
                        start += Vector2Int.Down;
                        break;
                    case '<':
                        start += Vector2Int.Left;
                        break;
                    case '>':
                        start += Vector2Int.Right;
                        break;
                }
                visited.Add(start);
            }


            return visited.Count;
        }

        public override object Task2()
        {
            HashSet<Vector2Int> visited = new HashSet<Vector2Int>();
            Vector2Int santa = Vector2Int.Zero;
            Vector2Int robot = Vector2Int.Zero;

            ref Vector2Int actor = ref santa;
            for (int i = 0; i < rawInput.Length; i++)
            {
                if (i % 2 == 0)
                {
                    actor = ref santa;
                }
                else
                {
                    actor = ref robot;
                }
                char c = rawInput[i];
                switch (c)
                {
                    case '^':
                        actor += Vector2Int.Up;
                        break;
                    case 'v':
                        actor += Vector2Int.Down;
                        break;
                    case '<':
                        actor += Vector2Int.Left;
                        break;
                    case '>':
                        actor += Vector2Int.Right;
                        break;
                }
                visited.Add(actor);
            }
            return visited.Count;
        }
    }
}
