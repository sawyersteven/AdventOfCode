using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge20 : Challenge
    {
        public override object Task1()
        {
            int match = int.Parse(rawInput) / 10;

            for (int i = 1; ; i++)
            {
                if (SumDivisors(i) >= match) return i;
            }
        }

        private int SumDivisors(int val)
        {
            int sum = 0;
            int end = (int)Math.Sqrt(val) + 1;
            for (int i = 1; i <= end; i++)
            {
                if (val % i == 0)
                {
                    sum += i;
                    if (i != val / i) sum += val / i;
                }
            }
            return sum;
        }

        public override object Task2()
        {
            int match = int.Parse(rawInput);

            for (int i = 1; ; i++)
            {
                if (SumDivisors2(i) >= match) return i;
            }
        }

        private int SumDivisors2(int val)
        {
            int sum = 0;
            int end = (int)Math.Sqrt(val) + 1;
            for (int i = 1; i <= end; i++)
            {
                if (val % i == 0)
                {
                    if (val / i <= 50) sum += i;
                    if (i <= 50) sum += val / i;
                }
            }
            return sum * 11;
        }
    }
}
