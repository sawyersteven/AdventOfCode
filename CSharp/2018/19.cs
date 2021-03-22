using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge19 : Challenge
    {
        (string, int[])[] code;
        int ip;
        public override object Task1()
        {
            ip = int.Parse(input[0].Split(' ')[1]);

            code = new (string, int[])[input.Length - 1];
            for (int i = 0; i < code.Length; i++)
            {
                (string, int[]) inst = (input[i + 1].Substring(0, 4), null);

                inst.Item2 = Array.ConvertAll(input[i + 1].Substring(5).Split(' '), int.Parse);

                code[i] = inst;
            }

            int[] mem = new int[6];
            ref int instPtr = ref mem[ip];

            while (instPtr < code.Length)
            {
                Challenge16.Computer.OperationsByName[code[instPtr].Item1](mem, code[instPtr].Item2);
                instPtr++;
            }

            return mem[0];
        }

        // This takes appx 4 days to calculate the prime factors of a large number and add them up.
        // This can be largely skipped by waiting for the last register to stabilize, finding the
        // prime factors the easy way, and adding them up myself.
        public override object Task2()
        {
            int[] mem = new int[] { 1, 0, 0, 0, 0, 0 };
            ref int instPtr = ref mem[ip];

            int lastReg = mem[5];
            for (int i = 1; ; i++)
            {
                Challenge16.Computer.OperationsByName[code[instPtr].Item1](mem, code[instPtr].Item2);
                instPtr++;
                if (i % 10 == 0)
                {
                    if (lastReg == mem[5]) break;
                    lastReg = mem[5];
                }
            }

            int t = 0;
            foreach (int i in PrimeFactors(mem[5])) t += i;

            return t;
        }

        private List<int> PrimeFactors(int n)
        {
            List<int> f = new List<int>();
            int i = 1;
            while (i * i <= n)
            {
                if (n % i == 0)
                {
                    f.Add(i);
                    if (n / i != i)
                    {
                        f.Add(n / i);
                    }
                }

                i++;
            }
            return f;
        }
    }
}