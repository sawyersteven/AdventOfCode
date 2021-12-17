using System;
using System.Collections.Generic;
using AdventOfCode;
using ExtensionMethods;
using Grids;

namespace Advent2021
{
    using node = ValueTuple<Vector2Int, int>;

    public class Challenge15 : Challenge
    {
        int[,] map;
        public override void ParseInput()
        {
            map = Utils.InputToIntArray(input, true, 0);
        }

        public override object Task1()
        {
            return FindPath(map);
        }

        private int FindPath(int[,] map)
        {
            Vector2Int start = new Vector2Int(1, 1);
            Vector2Int end = new Vector2Int(map.GetLength(0) - 2, map.GetLength(1) - 2);

            PriorityQueue<Vector2Int, int> pq = new PriorityQueue<Vector2Int, int>();
            Dictionary<Vector2Int, int> scores = new Dictionary<Vector2Int, int>();
            pq.Enqueue(start, 0);
            scores[start] = 0;

            while (pq.Count > 0)
            {
                Vector2Int currentPos = pq.Dequeue();
                int currentScore = scores[currentPos];

                if (currentPos == end) return currentScore;

                foreach (Vector2Int v in Vector2Int.CardinalDirections)
                {
                    Vector2Int nextPos = currentPos + v;
                    if (map[nextPos.y, nextPos.x] == 0) continue;
                    int nextScore = currentScore + map[nextPos.y, nextPos.x];

                    if (!scores.TryGetValue(nextPos, out int existingScore) || existingScore > nextScore)
                    {
                        scores[nextPos] = nextScore;
                        pq.Enqueue(nextPos, nextScore);

                    }
                }
            }
            return -1;
        }


        private int[,] MultiplyMap(int[,] map)
        {
            const int mapMultiple = 5;
            int my = map.GetLength(0) - 2;
            int mx = map.GetLength(1) - 2;
            int[,] bigMap = new int[input.Length * mapMultiple + 2, input[0].Length * mapMultiple + 2];

            for (int yMult = 0; yMult < mapMultiple; yMult++)
            {
                for (int xMult = 0; xMult < mapMultiple; xMult++)
                {
                    for (int y = 0; y < my; y++)
                    {
                        for (int x = 0; x < mx; x++)
                        {
                            int v = map[y + 1, x + 1];
                            v += yMult + xMult;
                            if (v > 9) v %= 9;
                            bigMap[y + (yMult * my) + 1, x + (xMult * mx) + 1] = v;
                        }
                    }
                }
            }
            return bigMap;
        }

        public override object Task2()
        {
            return FindPath(MultiplyMap(map));
        }
    }
}
