using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2018
{
    public class Challenge03 : Challenge
    {
        private struct Claim
        {
            public int ID;
            public int X;
            public int Y;
            public int X2;
            public int Y2;
        }

        private Claim[] claims;
        public override void ParseInput()
        {
            Claim[] claims = new Claim[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                Claim claim = new Claim();
                string line = input[i];
                string[] parts = line.Split(' ');
                string[] xy = parts[2].Substring(0, parts[2].Length - 1).Split(',');
                string[] wh = parts[parts.Length - 1].Split('x');
                claim.X = int.Parse(xy[0]);
                claim.Y = int.Parse(xy[1]);
                claim.X2 = claim.X + int.Parse(wh[0]);
                claim.Y2 = claim.Y + int.Parse(wh[1]);
                claim.ID = int.Parse(parts[0].Substring(1));
                claims[i] = claim;
            }
        }

        HashSet<(int, int)> collisions;
        public override object Task1()
        {
            HashSet<(int, int)> sqins = new HashSet<(int, int)>();
            collisions = new HashSet<(int, int)>();

            foreach (Claim c in claims)
            {
                (int, int) sqin = (0, 0);
                for (int row = c.Y; row < c.Y2; row++)
                {
                    sqin.Item1 = row;
                    for (int col = c.X; col < c.X2; col++)
                    {
                        sqin.Item2 = col;
                        if (sqins.Contains(sqin))
                        {
                            collisions.Add(sqin);
                        }
                        else
                        {
                            sqins.Add(sqin);
                        }
                    }
                }
            }
            return collisions.Count;
        }

        public override object Task2()
        {
            foreach (Claim c in claims)
            {
                bool collision = false;
                (int, int) sqin = (0, 0);
                for (int row = c.Y; row < c.Y2; row++)
                {
                    sqin.Item1 = row;
                    for (int col = c.X; col < c.X2; col++)
                    {
                        sqin.Item2 = col;
                        if (collisions.Contains(sqin))
                        {
                            collision = true;
                            break;
                        }
                    }
                    if (collision) break;
                }
                if (!collision) return c.ID;
            }
            return null;
        }
    }
}