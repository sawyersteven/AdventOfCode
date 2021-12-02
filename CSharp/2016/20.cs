using System;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge20 : Challenge
    {
        private (uint, uint)[] ranges;
        public override void ParseInput()
        {
            ranges = new (uint, uint)[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                string[] parts = input[i].Split('-');
                ranges[i] = (uint.Parse(parts[0]), uint.Parse(parts[1]));
            }
            Array.Sort(ranges);
        }

        public override object Task1()
        {
            uint high = ranges[0].Item2;
            for (int i = 1; i < ranges.Length; i++)
            {
                if (ranges[i].Item1 > high + 1)
                {
                    break;
                }
                if (ranges[i].Item2 > high)
                {
                    high = ranges[i].Item2;
                }
            }
            return high + 1;
        }

        public override object Task2()
        {
            uint banned = 0;

            (uint rangeStart, uint rangeEnd) = ranges[0];
            for (int i = 0; i < ranges.Length && rangeEnd < uint.MaxValue; i++)

            {
                (uint low, uint high) = ranges[i];
                if (low > rangeEnd + 1)
                {
                    banned += (rangeEnd - rangeStart) + 1;
                    (rangeStart, rangeEnd) = (low, high);
                    if (rangeEnd == uint.MaxValue) break;
                }
                if (high > rangeEnd)
                {
                    rangeEnd = high;
                }
            }
            banned += (rangeEnd - rangeStart) + 1;

            return uint.MaxValue - banned + 1;
        }
    }
}
