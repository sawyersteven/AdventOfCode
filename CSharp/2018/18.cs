using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge18 : Challenge
    {
        private const char open = '.';
        private const char tree = '|';
        private const char lumber = '#';

        private const int minutes = 10;
        public override object Task1()
        {
            char[,] forest = Utils.InputToCharArray(input, true);

            for (int i = 0; i < minutes; i++)
            {
                forest = RunIteration(forest);
            }

            return ResourceValue(forest);
        }

        private char[,] RunIteration(char[,] forest)
        {
            char[,] next = forest.Clone() as char[,];
            for (int y = 1; y < input.Length + 1; y++)
            {
                for (int x = 1; x < input[0].Length + 1; x++)
                {
                    char[] surrounding = new char[]{forest[y - 1, x - 1],
                                                    forest[y - 1, x],
                                                    forest[y - 1, x + 1],
                                                    forest[y, x - 1],
                                                    forest[y, x + 1],
                                                    forest[y + 1, x - 1],
                                                    forest[y + 1, x],
                                                    forest[y + 1, x + 1]};

                    if (forest[y, x] == open)
                    {
                        int t = 0;
                        foreach (char c in surrounding)
                        {
                            if (c == tree) t++;
                        }
                        next[y, x] = (t >= 3) ? tree : open;
                    }
                    else if (forest[y, x] == tree)
                    {
                        int l = 0;
                        foreach (char c in surrounding)
                        {
                            if (c == lumber) l++;
                        }
                        next[y, x] = (l >= 3) ? lumber : tree;
                    }
                    else if (forest[y, x] == lumber)
                    {
                        int l = 0;
                        int t = 0;
                        foreach (char c in surrounding)
                        {
                            if (c == lumber) l++;
                            else if (c == tree) t++;
                        }

                        next[y, x] = (l > 0 && t > 0) ? lumber : open;
                    }
                }
            }
            return next;
        }

        private int ResourceValue(char[,] forest)
        {
            int woods = 0;
            int yards = 0;
            for (int y = 1; y < input.Length + 1; y++)
            {
                for (int x = 1; x < input[0].Length + 1; x++)
                {
                    if (forest[y, x] == tree) woods++;
                    else if (forest[y, x] == lumber) yards++;
                }
            }
            return woods * yards;
        }

        public override object Task2()
        {
            const int minutes = 1_000_000_000;
            const int patternSearchOffset = 1000;
            const int sampleDataLen = 75;

            char[,] forest = Utils.InputToCharArray(input, true);

            // go long enough to find a stable pattern
            for (int _ = 0; _ < patternSearchOffset; _++)
            {
                forest = RunIteration(forest);
            }

            int patternStart = ResourceValue(forest);

            int[] sampleData = new int[sampleDataLen];
            for (int i = 0; i < sampleDataLen; i++)
            {
                forest = RunIteration(forest);
                sampleData[i] = ResourceValue(forest);
            }

            int patternLength = Array.IndexOf(sampleData, patternStart) + 1;
            if (patternLength == 0 || Array.LastIndexOf(sampleData, patternStart) == patternLength - 1) return "Pattern length not found -- increase sample size or sample offset";

            return sampleData[(minutes - patternSearchOffset - 1) % patternLength];
        }
    }
}