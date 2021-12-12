using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    public class Challenge11 : Challenge
    {
        private const int gridSize = 10;

        int[,] octopuses;
        public override void ParseInput()
        {
            octopuses = Utils.InputToIntArray(input, true, 0);
        }

        public override object Task1()
        {
            int[,] octs = octopuses.Duplicate();

            int count = 0;
            for (int _ = 0; _ < 100; _++)
            {
                count += SimulateTurn(octs);
            }
            return count;
        }

        private int SimulateTurn(int[,] octs)
        {
            HashSet<Vector2Int> flashedThisRound = new HashSet<Vector2Int>();
            Queue<Vector2Int> q = new Queue<Vector2Int>();

            int flashCount = 0;

            Vector2Int current = Vector2Int.Zero;
            for (current.y = 1; current.y < gridSize + 1; current.y++)
            {
                for (current.x = 1; current.x < gridSize + 1; current.x++)
                {
                    octs[current.y, current.x]++;
                }
            }

            while (true)
            {
                for (current.y = 1; current.y < gridSize + 1; current.y++)
                {
                    for (current.x = 1; current.x < gridSize + 1; current.x++)
                    {
                        if (octs[current.y, current.x] > 9)
                        {
                            q.Enqueue(current);
                        }
                    }
                }

                if (q.Count == 0) break;
                flashCount += q.Count;

                while (q.Count > 0)
                {

                    Vector2Int flashing = q.Dequeue();
                    foreach (Vector2Int v in Vector2Int.AllDirections)
                    {
                        Vector2Int n = flashing + v;
                        octs[n.y, n.x]++;
                    }
                    octs[flashing.y, flashing.x] = 0;
                    flashedThisRound.Add(flashing);
                }
            }
            foreach (Vector2Int v in flashedThisRound)
            {
                octs[v.y, v.x] = 0;
            }

            return flashCount;
        }

        // Haven't a clue why this runs so much quicker than part one
        public override object Task2()
        {
            int[,] octs = octopuses.Duplicate();

            for (int i = 1; ; i++)
            {
                if (SimulateTurn(octs) == gridSize * gridSize) return i;
            }
        }
    }
}
