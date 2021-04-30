using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge08 : Challenge
    {
        public override object Task1()
        {
            int t = 0;
            foreach (string line in input)
            {
                int escapedChars = 0;
                for (int i = 0; i < line.Length; i++)
                {
                    if (line[i] == '\\')
                    {
                        if (line[i + 1] == '\\')
                        {
                            escapedChars++;
                            i++;
                        }
                        else if (line[i + 1] == '"')
                        {
                            escapedChars++;
                            i++;
                        }
                        else if (line[i + 1] == 'x')
                        {
                            escapedChars += 3;
                            i += 3;
                        }
                    }
                }
                t += escapedChars + 2;
            }
            return t;
        }

        public override object Task2()
        {
            int t = 0;
            foreach (string line in input)
            {
                t += 2;
                foreach (char c in line)
                {
                    if (c == '\\' || c == '"') t++;
                }

            }
            return t;
        }
    }
}
