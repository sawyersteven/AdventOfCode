using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge06 : Challenge
    {
        private List<int> lanternFish;
        public override void ParseInput()
        {
            string[] parts = rawInput.Split(',');
            lanternFish = new List<int>(parts.Length);
            foreach (string s in parts) lanternFish.Add(int.Parse(s));
        }

        public override object Task1()
        {
            List<int> lf = new List<int>(lanternFish);
            for (int day = 0; day < 80; day++)
            {
                int count = lf.Count;
                for (int i = 0; i < count; i++)
                {
                    if (lf[i] == 0)
                    {
                        lf.Add(8);
                        lf[i] = 6;
                    }
                    else
                    {
                        lf[i]--;
                    }
                }
            }

            return lf.Count;

        }

        // obviously brute forcing this isn't going to work, and I'm kind of proud of this solution
        public override object Task2()
        {
            long[] counts = new long[10];

            foreach (int i in lanternFish)
            {
                counts[i]++;
            }

            for (int day = 0; day < 256; day++)
            {
                counts[9] = counts[0];
                counts[7] += counts[0];
                for (int i = 0; i < counts.Length - 1; i++)
                {
                    counts[i] = counts[i + 1];
                }
            }

            long sum = 0;
            for (int i = 0; i < counts.Length - 1; i++)
            {
                sum += counts[i];
            }

            return sum;
        }
    }
}
