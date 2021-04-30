using System;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2015
{
    public class Challenge25 : Challenge
    {
        int row;
        int col;
        const ulong seed = 20151125;

        private void ParseInput()
        {
            string[] parts = rawInput.Split(new char[] { ' ', '.', ',' }, StringSplitOptions.RemoveEmptyEntries);
            col = int.Parse(parts[^1]);
            row = int.Parse(parts[^3]);
        }

        public override object Task1()
        {
            ParseInput();

            int d = Distance(row, col);
            Console.WriteLine(d);

            ulong code = seed;
            for (int i = 0; i < d; i++)
            {
                code = (code * 252533) % 33554393;
            }

            return code;
        }

        private int Distance(int row, int col)
        {
            int d = col - 1;
            for (int i = 0; i < row + col - 1; i++)
            {
                d += i;
            }
            return d;
        }

        public override object Task2()
        {
            return null;
        }
    }
}
