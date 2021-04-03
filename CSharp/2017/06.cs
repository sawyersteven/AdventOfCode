using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge06 : Challenge
    {
        HashSet<string> history;
        int[] mem;
        public override object Task1()
        {
            mem = Array.ConvertAll(rawInput.Split('\t'), int.Parse);
            history = new HashSet<string>();

            for (int cycle = 1; ; cycle++)
            {
                RunCycle(mem);

                string id = string.Join(',', mem);
                if (history.Contains(id)) return cycle;

                history.Add(id);
            }
        }

        private void RunCycle(int[] mem)
        {
            int ind = 0;
            int num = 0;
            for (int i = 0; i < mem.Length; i++)
            {
                if (mem[i] > num)
                {
                    num = mem[i];
                    ind = i;
                }
            }

            mem[ind] = 0;

            for (; num > 0; num--)
            {
                ind = (ind + 1) % mem.Length;
                mem[ind]++;
            }
        }

        public override object Task2()
        {
            string target = string.Join(',', mem);

            for (int i = 1; ; i++)
            {
                RunCycle(mem);
                if (string.Join(',', mem) == target) return i;
            }
        }
    }
}
