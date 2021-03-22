using AdventOfCode;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Advent2020
{
    public class Challenge09 : Challenge
    {
        const int preambleLen = 25;
        long[] intInput;
        long T1Answer;

        private bool SumExists(IEnumerable<Int64> nums, Int64 sum)
        {
            foreach (int a in nums)
            {
                Int64 b = sum - a;
                if (a != b && nums.Any(x => x == b))
                {
                    return true;
                }
            }
            return false;
        }

        public override object Task1()
        {
            intInput = Array.ConvertAll(input, long.Parse);

            int preambleStart = 0;

            for (int currentPos = preambleLen; currentPos < input.Length; currentPos++)
            {
                ArraySegment<long> preambSlice = new ArraySegment<long>(intInput, preambleStart, preambleLen); //intInput.GetRange(preambleStart, preambleLen);
                if (!SumExists(preambSlice, intInput[currentPos]))
                {
                    T1Answer = intInput[currentPos];
                    return T1Answer;
                };
                preambleStart++;
            }

            return null;
        }

        public override object Task2()
        {
            for (int i = 0; i < intInput.Length; i++)
            {
                Int64 t = 0;
                int j = i;
                while (t < T1Answer)
                {
                    t += intInput[j];
                    j++;
                }
                if (t == T1Answer)
                {
                    ArraySegment<long> range = new ArraySegment<long>(intInput, i, i - j);
                    return range.Min() + range.Max();
                }
            }

            return null;
        }
    }
}