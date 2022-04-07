using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Text;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge12 : Challenge
    {
        public override object Task1()
        {
            return CountNums(rawInput);
        }

        private int CountNums(string input)
        {
            int t = 0;
            string[] parts = input.Split(new char[] { ':', ',', '}', '[', ']' });
            foreach (string s in parts)
            {
                if (int.TryParse(s, out int i)) t += i;
            }
            return t;
        }

        public override object Task2()
        {
            List<(int, int)> removeRanges = new List<(int, int)>();
            for (int i = rawInput.Length - 5; i > 0; i--)
            {
                if (rawInput[i] != ':') continue;

                // find `:"red"` then iterate forward and backward to find
                // length of enclosing {}
                if (rawInput.Substring(i, 6) == ":\"red\"")
                {
                    int start = i;
                    int end = i;

                    int depth = 1;
                    while (start > 0)
                    {
                        start--;
                        depth += rawInput[start] == '}' ? 1 : 0;

                        if (rawInput[start] == '{')
                        {
                            depth--;
                            if (depth == 0) break;
                        }
                    }

                    depth = 1;
                    while (end < rawInput.Length)
                    {
                        end++;
                        depth += rawInput[end] == '}' ? -1 : 0;
                        depth += rawInput[end] == '{' ? 1 : 0;

                        if (depth == 0) break;
                    }
                    removeRanges.Add((start, end));
                    i = start;
                }
            }

            removeRanges.Reverse();

            // remove nested ranges
            for (int i = 0; i < removeRanges.Count; i++)
            {
                for (int j = i + 1; j < removeRanges.Count; j++)
                {
                    if (removeRanges[j].Item2 < removeRanges[i].Item2)
                    {
                        removeRanges.RemoveAt(j);
                        j--;
                    }
                }
            }

            int ind = 0;
            StringBuilder sb = new StringBuilder();
            foreach ((int start, int end) in removeRanges)
            {
                sb.Append(rawInput.Substring(ind, start - ind));
                ind = end + 1;
            }
            sb.Append(rawInput.Substring(ind));

            return CountNums(sb.ToString());
        }
    }
}
