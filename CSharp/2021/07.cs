using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge07 : Challenge
    {
        int[] crabs;
        public override void ParseInput()
        {
            string[] parts = rawInput.Split(',');
            crabs = new int[parts.Length];
            for (int i = 0; i < parts.Length; i++)
            {
                crabs[i] = int.Parse(parts[i]);
            }
        }

        public override object Task1()
        {
            int least = int.MaxValue;

            foreach (int location in crabs)
            {
                int count = 0;
                foreach (int i in crabs)
                {
                    count += Math.Abs(location - i);
                    if (count > least) break;
                }
                if (count < least) least = count;
            }

            return least;
        }

        public override object Task2()
        {
            int least = int.MaxValue;

            int range = 0; // part one worked without this, mostly by coincidence. And I know Linq has Max...
            foreach (int crab in crabs)
            {
                if (crab > range) range = crab;
            }

            for (int location = 0; location < range; location++)
            {
                int count = 0;
                foreach (int i in crabs)
                {
                    int N = Math.Abs(location - i);
                    count += (N * (N + 1)) / 2; // Gauss
                    if (count > least) break;
                }
                if (count < least) least = count;
            }

            return least;
        }
    }
}
