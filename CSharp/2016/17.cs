using System.Collections.Generic;
using AdventOfCode;
using Grids;

namespace Advent2016
{
    public class Challenge17 : Challenge
    {
        private const int mapSize = 4;
        private const string passcode = "qzthpkfp";
        private readonly System.Security.Cryptography.MD5 md5 = System.Security.Cryptography.MD5.Create();

        private readonly char[] charDirections = new char[] { 'U', 'D', 'L', 'R' };
        private readonly Vector2Int[] v2Directions = new Vector2Int[] { Vector2Int.Down, Vector2Int.Up, Vector2Int.Left, Vector2Int.Right };

        private readonly Vector2Int start = new Vector2Int(1, 1);
        private readonly Vector2Int end = new Vector2Int(mapSize, mapSize);

        public override object Task1()
        {
            Queue<(Vector2Int, string)> q = new Queue<(Vector2Int, string)>();
            q.Enqueue((start, string.Empty));

            while (q.Count > 0)
            {
                (Vector2Int location, string steps) = q.Dequeue();

                if (location == end) return string.Join(null, steps);

                string hash = CreateMD5(passcode + steps);
                for (int i = 0; i < 4; i++)
                {
                    if (hash[i] >= 'b' && hash[i] <= 'f')
                    {
                        Vector2Int next = location + v2Directions[i];
                        if (next.x == 0 || next.y == 0 || next.x > mapSize || next.y > mapSize) continue;
                        string nextSteps = steps + charDirections[i];
                        q.Enqueue((next, nextSteps));
                    }
                }
            }


            return null;
        }

        private string CreateMD5(string input, int iters = 1)
        {
            byte[] bytes = System.Text.Encoding.ASCII.GetBytes(input);
            for (int _ = 0; _ < iters; _++)
            {
                bytes = ToLowercaseHexBytes(md5.ComputeHash(bytes));
            }
            return System.Text.Encoding.ASCII.GetString(bytes);
        }

        private byte[] ToLowercaseHexBytes(byte[] hash)
        {
            byte[] ret = new byte[32];
            for (int i = 0, j = 0; i < hash.Length; i++, j += 2)
            {
                ret[j] = (byte)((hash[i] / 16) + 48);
                // some bit-fiddling to see if the value is over 57 so it can have
                // 39 added to it in order to get a lower-case ascii letter value
                // This only works on unsigned values.
                ret[j] += (byte)((57 + (~ret[j] + 1) >> 31 & 1) * 39);
                ret[j + 1] = (byte)((hash[i] % 16) + 48);
                ret[j + 1] += (byte)((57 + (~ret[j + 1] + 1) >> 31 & 1) * 39);
            }
            return ret;
        }

        public override object Task2()
        {
            Queue<(Vector2Int, string)> q = new Queue<(Vector2Int, string)>();
            q.Enqueue((start, string.Empty));

            int longest = 0;

            while (q.Count > 0)
            {
                (Vector2Int location, string steps) = q.Dequeue();

                if (location == end)
                {
                    if (steps.Length > longest) longest = steps.Length;
                    continue;
                }

                string hash = CreateMD5(passcode + steps);
                for (int i = 0; i < 4; i++)
                {
                    if (hash[i] >= 'b' && hash[i] <= 'f')
                    {
                        Vector2Int next = location + v2Directions[i];
                        if (next.x == 0 || next.y == 0 || next.x > mapSize || next.y > mapSize) continue;
                        string nextSteps = steps + charDirections[i];
                        q.Enqueue((next, nextSteps));
                    }
                }
            }

            return longest;

        }
    }
}
