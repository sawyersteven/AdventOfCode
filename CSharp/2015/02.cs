using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge02 : Challenge
    {
        public override object Task1()
        {
            int total = 0;
            foreach (string line in input)
            {
                int[] dims = Array.ConvertAll(line.Split('x'), int.Parse);
                Array.Sort(dims);
                total += dims[0] * dims[1] * 3 + dims[1] * dims[2] * 2 + dims[2] * dims[0] * 2;
            }
            return total;
        }

        public override object Task2()
        {
            int total = 0;
            foreach (string line in input)
            {
                int[] dims = Array.ConvertAll(line.Split('x'), int.Parse);
                Array.Sort(dims);
                total += dims[0] * 2 + dims[1] * 2;
                total += dims[0] * dims[1] * dims[2];
            }
            return total;
        }
    }
}
