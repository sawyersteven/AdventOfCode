using System;
using AdventOfCode;

namespace Advent2019
{
    public class Challenge01 : Challenge
    {
        public override object Task1()
        {
            long total = 0;
            foreach (string line in input)
            {
                total += int.Parse(line) / 3 - 2;
            }

            return total;
        }

        public override object Task2()
        {
            long total = 0;
            foreach (string line in input)
            {
                int mass = int.Parse(line);
                while (true)
                {
                    mass = mass / 3 - 2;
                    if (mass <= 0) break;
                    total += mass;
                }
            }
            return total;
        }
    }
}