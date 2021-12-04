using System;
using AdventOfCode;

namespace Advent2021
{
    // This felt like a good opportunity to work on my bit work. These might be
    // less than super readable, but very fast and fun to figure out.
    public class Challenge03 : Challenge
    {
        int[] inputNums;
        public override void ParseInput()
        {
            inputNums = new int[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                inputNums[i] = Convert.ToInt32(input[i], 2);
            }
        }

        public override object Task1()
        {
            int gamma = 0;
            int epsilon = 0;

            int mask = 1 << input[0].Length - 1;

            for (int _ = 0; _ < input[0].Length; _++)
            {
                int onesCount = 0;
                foreach (int num in inputNums)
                {
                    onesCount += (num & mask) == 0 ? 0 : 1;
                }
                gamma <<= 1;
                epsilon <<= 1;
                if (onesCount >= input.Length / 2) gamma++;
                else epsilon++;
                mask >>= 1;
            }

            return gamma * epsilon;
        }

        public override object Task2()
        {
            return T2Find(true) * T2Find(false);
        }

        private int T2Find(bool isGenerator)
        {
            int rating = 0;
            int lastValid = 0;
            int mask = 1 << input[0].Length - 1;
            for (int b = input[0].Length - 1; b >= 0; b--)
            {
                int onesCount = 0;
                int numCount = 0;

                rating <<= 1;
                foreach (int num in inputNums)
                {
                    if (num >> (b + 1) == rating >> 1)
                    {
                        numCount++;
                        onesCount += (num >> b) & 0b1;
                        lastValid = num;
                    };
                }

                if (numCount == 1) return lastValid;

                if (isGenerator) rating += (onesCount >= numCount / 2f) ? 1 : 0;
                else rating += (onesCount >= numCount / 2f) ? 0 : 1;

            }
            return rating;
        }
    }
}
