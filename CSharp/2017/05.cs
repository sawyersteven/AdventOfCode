using System;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge05 : Challenge
    {
        int[] instructions;
        public override object Task1()
        {
            instructions = Array.ConvertAll(input, int.Parse);
            int location = 0;
            int steps = 0;
            for (; ; steps++)
            {
                if (location < 0 || location > instructions.Length - 1)
                {
                    break;
                }
                ref int inst = ref instructions[location];
                location += instructions[location];
                inst++;
            }

            return steps;
        }

        public override object Task2()
        {
            instructions = Array.ConvertAll(input, int.Parse);
            int location = 0;
            int steps = 0;
            for (; ; steps++)
            {
                if (location < 0 || location > instructions.Length - 1)
                {
                    break;
                }
                ref int inst = ref instructions[location];
                location += instructions[location];
                if (inst >= 3) inst--;
                else inst++;
            }

            return steps;
        }
    }
}
