using System;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2016
{
    public class Challenge18 : Challenge
    {
        private const char safe = '.';
        private const char trap = '^';

        public override object Task1()
        {
            return CountSafe(40);
        }

        private int CountSafe(int rows)
        {
            int count = rows * rawInput.Length;

            char[] prev = rawInput.ToCharArray();
            char[] current = new char[rawInput.Length];

            count -= prev.Count(trap);

            for (int y = 1; y < rows; y++)
            {
                for (int x = 0; x < rawInput.Length; x++)
                {
                    byte b = 0b000;
                    for (int z = -1; z < 2; z++)
                    {
                        b <<= 1;
                        if (x + z > rawInput.Length - 1 || x + z < 0) continue;
                        b |= (byte)(prev[x + z] == trap ? 1 : 0);
                    }
                    if (b == 0b110 || b == 0b011 || b == 0b100 || b == 0b001)
                    {
                        current[x] = trap;
                        count--;
                    }
                }
                Array.Copy(current, prev, current.Length);
                Array.Clear(current, 0, current.Length);
            }
            return count;
        }

        public override object Task2()
        {
            return CountSafe(400_000);
        }
    }
}
