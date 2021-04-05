using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge13 : Challenge
    {
        SortedDictionary<int, int> ranges;
        public override object Task1()
        {
            ranges = new SortedDictionary<int, int>();
            foreach (string line in input)
            {
                int[] parts = Array.ConvertAll(line.Split(':'), int.Parse);
                ranges[parts[0]] = parts[1];
            }
            return CalcSeverity();
        }

        private bool ScannerAtZero(int time, int range)
        {
            int rf = (range * 2) - 2;
            int index = time % rf;
            return index == 0 || (rf - index) == 0;
        }

        private int CalcSeverity()
        {
            int severity = 0;
            foreach ((int time, int range) in ranges)
            {
                if (ScannerAtZero(time, range))
                {
                    severity += time * (ranges[time]);
                }
            }
            return severity;
        }

        private bool CaughtByScanner(int offset)
        {
            foreach ((int time, int range) in ranges)
            {
                if (ScannerAtZero(time + offset, range))
                {
                    return true;
                }
            }
            return false;
        }

        public override object Task2()
        {
            for (int offset = 0; ; offset++)
            {
                /* have to check the zero position specificially because it
                only raise the severity by a multiple of zero, which doesn't
                tell if we are caught or not

                There's probably a quicker, more math-y way to solve this.
                */
                if (ScannerAtZero(offset, ranges[0])) continue;

                if (!CaughtByScanner(offset)) return offset;
            }
        }
    }
}
