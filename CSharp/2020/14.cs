using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge14 : Challenge
    {
        public override object Task1()
        {
            Dictionary<int, long> mem = new Dictionary<int, long>();
            long maxVal = (long)Math.Pow(2, 36);

            long maskOnes = 0;
            long maskZeros = maxVal;
            string mask;

            for (int i = 0; i < input.Length; i++)
            {

                if (input[i].Length == 43)
                {
                    mask = input[i].Substring(7);
                    maskOnes = Convert.ToInt64(mask.Replace('X', '0'), 2);
                    maskZeros = Convert.ToInt64(mask.Replace('X', '1'), 2);
                    continue;
                }

                // regex is significantly slower for this
                int memAddr = int.Parse(input[i].Substring(4).Split(']')[0]);
                long val = long.Parse(input[i].Split("= ")[1]);
                val |= maskOnes;
                val &= maskZeros;

                mem[memAddr] = val;
            }

            long answer = 0;
            foreach (long v in mem.Values)
            {
                answer += v;
            }
            return answer;
        }

        public override object Task2()
        {
            Dictionary<long, long> mem = new Dictionary<long, long>();
            long maxVal = (long)Math.Pow(2, 36);
            string mask = string.Empty;

            for (int i = 0; i < input.Length; i++)
            {
                if (input[i].Length == 43)
                {
                    mask = input[i].Substring(7);
                    continue;
                }

                // regex is significantly slower for this
                char[] memAddr = Convert.ToString(int.Parse(input[i].Substring(4).Split(']')[0]), 2).PadLeft(36, '0').ToCharArray();
                int val = int.Parse(input[i].Split("= ")[1]);

                foreach (long addr in MemAddresses(mask, memAddr)) mem[addr] = val;
            }

            long answer = 0;
            foreach (long v in mem.Values)
            {
                answer += v;
            }
            return answer;
        }

        private long[] MemAddresses(string mask, char[] baseAddr)
        {
            List<int> xIndexes = new List<int>();

            for (int i = 0; i < mask.Length; i++)
            {
                if (mask[i] == 'X')
                {
                    xIndexes.Add(i);
                }
                else if (mask[i] == '1') baseAddr[i] = '1';
            }

            long[] newAddrs = new long[(int)Math.Pow(2, xIndexes.Count)];

            for (int p = 0; p < newAddrs.Length; p++)
            {
                for (int i = 0; i < xIndexes.Count; i++)
                {
                    baseAddr[xIndexes[i]] = ((p >> i) & 1) == 1 ? '1' : '0';
                }
                newAddrs[p] = Convert.ToInt64(string.Join("", baseAddr), 2);
            }

            return newAddrs;
        }
    }
}