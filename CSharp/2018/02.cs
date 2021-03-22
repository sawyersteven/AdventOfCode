using AdventOfCode;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge02 : Challenge
    {

        public override object Task1()
        {
            int twos = 0;
            int threes = 0;
            foreach (string line in input)
            {
                Dictionary<char, int> d = new Dictionary<char, int>();
                foreach (char c in line)
                {
                    if (!d.ContainsKey(c)) d[c] = 0;
                    d[c]++;
                }
                if (d.ContainsValue(3)) threes++;
                if (d.ContainsValue(2)) twos++;
            }

            return twos * threes;
        }

        public override object Task2()
        {
            for (int a = 0; a < input.Length - 1; a++)
            {
                for (int b = a + 1; b < input.Length; b++)
                {
                    if (Compare(input[a], input[b]))
                    {
                        return Filter(input[a], input[b]);
                    };
                }
            }

            return null;
        }

        private string Filter(string a, string b)
        {
            string s = "";
            for (int i = 0; i < a.Length; i++)
            {
                if (a[i] == b[i]) s += a[i];
            }
            return s;
        }

        private bool Compare(string a, string b)
        {
            bool diff = false;
            for (int i = 0; i < a.Length; i++)
            {
                if (a[i] != b[i])
                {
                    if (diff) return false;
                    diff = true;
                }
            }
            return diff;
        }
    }
}