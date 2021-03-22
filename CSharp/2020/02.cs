using AdventOfCode;
using System;

namespace Advent2020
{
    public class Challenge02 : Challenge
    {

        public override object Task1()
        {
            int goodPasswords = 0;

            foreach (string line in input)
            {
                string[] parts = line.Split(new char[] { ' ', '-', ':' });
                int rangeLow = int.Parse(parts[0]);
                int rangeHigh = int.Parse(parts[1]);
                char reqChar = parts[2][0];
                string pw = parts[4];

                int count = 0;
                foreach (char c in pw)
                {
                    if (c == reqChar)
                    {
                        count++;
                    }
                }

                if (count >= rangeLow && count <= rangeHigh) goodPasswords++;
            }
            return goodPasswords;
        }

        public override object Task2()
        {
            int goodPasswords = 0;
            foreach (string line in input)
            {
                string[] parts = line.Split(new char[] { ' ', '-', ':' });

                char reqChar = parts[2][0];
                string pw = parts[4];

                bool a = pw[int.Parse(parts[0]) - 1] == reqChar;
                bool b = pw[int.Parse(parts[1]) - 1] == reqChar;

                if (a ^ b) goodPasswords++;
            }
            return goodPasswords;
        }
    }
}