using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge05 : Challenge
    {
        private char[] vowels = new char[] { 'a', 'e', 'i', 'o', 'u' };
        private string[] forbidden = new string[] { "ab", "cd", "pq", "xy" };
        public override object Task1()
        {
            int count = 0;

            foreach (string line in input)
            {
                bool hasForbidden = false;
                foreach (string f in forbidden)
                {
                    if (line.Contains(f))
                    {
                        hasForbidden = true;
                        break;
                    }
                }

                int vowelCount = 0;
                bool hasDouble = false;
                if (Array.IndexOf(vowels, line[^1]) != -1) vowelCount++;
                for (int i = 0; i < line.Length - 1; i++)
                {
                    if (Array.IndexOf(vowels, line[i]) != -1) vowelCount++;
                    if (line[i] == line[i + 1])
                    {
                        hasDouble = true;
                    }
                }
                if (!hasForbidden && hasDouble && vowelCount >= 3) count++;
            }

            return count;
        }

        public override object Task2()
        {
            int count = 0;
            foreach (string line in input)
            {
                bool hasPair = false;
                bool hasABA = false;
                for (int i = 0; i < line.Length - 3; i++)
                {
                    for (int j = i + 2; j < line.Length - 1; j++)
                    {
                        if (line[i] == line[j] && line[i + 1] == line[j + 1])
                        {
                            hasPair = true;
                        }
                    }
                }
                for (int i = 0; i < line.Length - 2; i++)
                {
                    if (line[i] == line[i + 2])
                    {
                        hasABA = true;
                    }
                }

                if (hasABA && hasPair) count++;
            }

            return count;
        }
    }
}
