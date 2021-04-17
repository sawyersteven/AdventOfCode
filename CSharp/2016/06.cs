using System.Collections.Generic;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge06 : Challenge
    {
        public override object Task1()
        {
            char[] corrected = new char[input[0].Length];

            Dictionary<char, int> counts = new Dictionary<char, int>();
            for (int i = 0; i < corrected.Length; i++)
            {
                counts.Clear();
                foreach (string line in input)
                {
                    if (!counts.ContainsKey(line[i])) counts[line[i]] = 0;
                    counts[line[i]]++;
                }

                int bestCount = 0;
                char bestChar = ' ';
                foreach (var kv in counts)
                {
                    if (kv.Value > bestCount)
                    {
                        bestCount = kv.Value;
                        bestChar = kv.Key;
                    }
                }
                corrected[i] = bestChar;

            }
            return string.Join("", corrected);
        }

        public override object Task2()
        {
            char[] corrected = new char[input[0].Length];

            Dictionary<char, int> counts = new Dictionary<char, int>();
            for (int i = 0; i < corrected.Length; i++)
            {
                counts.Clear();
                foreach (string line in input)
                {
                    if (!counts.ContainsKey(line[i])) counts[line[i]] = 0;
                    counts[line[i]]++;
                }

                int worstCount = int.MaxValue;
                char worstChar = ' ';
                foreach (var kv in counts)
                {
                    if (kv.Value < worstCount)
                    {
                        worstCount = kv.Value;
                        worstChar = kv.Key;
                    }
                }
                corrected[i] = worstChar;
            }
            return string.Join("", corrected);
        }
    }
}

