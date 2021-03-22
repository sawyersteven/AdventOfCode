using AdventOfCode;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge01 : Challenge
    {
        public override object Task1()
        {
            int sum = 0;
            foreach (string line in input)
            {
                sum += int.Parse(line);
            }
            return sum;
        }

        public override object Task2()
        {
            HashSet<int> h = new HashSet<int>();
            int[] freqs = new int[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                freqs[i] = int.Parse(input[i]);
            }

            int sum = 0;
            for (int ind = 0; ; ind++)
            {
                if (ind == freqs.Length) ind = 0;
                sum += freqs[ind];
                if (h.Contains(sum)) return sum;
                h.Add(sum);
            }
        }
    }
}