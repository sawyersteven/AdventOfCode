using System;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge15 : Challenge
    {

        private (int, int)[] discs;
        // size, turns from init to zero
        public override void ParseInput()
        {
            discs = new (int, int)[input.Length];
            for (int i = 0; i < input.Length; i++)
            {
                string[] parts = input[i].Split(' ');
                int p = int.Parse(parts[3]);
                int c = int.Parse(parts[^1].Substring(0, parts[^1].Length - 1));
                discs[i] = (p, c - p);
            }
        }

        public override object Task1()
        {
            return Run(discs);
        }

        /* This is basically 2020 day 13
        Add the LCM of each disc's size so far until this sum + droptime % currentDiscSize == 0
        where droptime is the index of the disc + 1.                                                                                                                                                                    

        Effectively finds the LCM of all the disc sizes offset by their dropTime + zeroOffset.
        */
        private long Run((int, int)[] discs)
        {
            long answer = 0;
            long lcm = 1;

            for (int i = 0; i < discs.Length; i++)
            {
                while ((answer + discs[i].Item2 + i + 1) % discs[i].Item1 != 0)
                {
                    answer += lcm;
                }
                lcm = LCM(lcm, discs[i].Item1);
            }

            return answer;
        }


        public override object Task2()
        {
            (int, int)[] discs2 = new (int, int)[input.Length + 1];
            Array.Copy(discs, discs2, discs.Length);
            discs2[^1] = (11, 0);
            return Run(discs2);
        }

        private long LCM(long a, long b)
        {
            (long big, long small) = a > b ? (a, b) : (b, a);

            for (long i = 1; i <= small; i++)
            {
                if ((big * i) % small == 0)
                {
                    return i * big;
                }
            }
            return small;
        }
    }
}
