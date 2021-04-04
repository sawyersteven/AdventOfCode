using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge08 : Challenge
    {
        // 0   1 2  3 4 5 6
        // b inc 5 if a > 1

        int highestEver = int.MinValue;
        public override object Task1()
        {
            Dictionary<string, int> registers = new Dictionary<string, int>();

            foreach (string line in input)
            {
                string[] parts = line.Split(' ');
                if (!registers.ContainsKey(parts[0])) registers[parts[0]] = 0;
                if (!registers.ContainsKey(parts[4])) registers[parts[4]] = 0;

                if (EvalCondition(registers[parts[4]], parts[5], int.Parse(parts[6])))
                {
                    registers[parts[0]] += int.Parse(parts[2]) * (parts[1][0] == 'i' ? 1 : -1);
                    if (registers[parts[0]] > highestEver) highestEver = registers[parts[0]];
                }
            }

            int maxval = int.MinValue;
            foreach (int i in registers.Values)
            {
                if (i > maxval) maxval = i;
            }
            return maxval;
        }

        private bool EvalCondition(int a, string op, int b)
        {
            return op switch
            {
                ">" => a > b,
                "<" => a < b,
                ">=" => a >= b,
                "<=" => a <= b,
                "==" => a == b,
                "!=" => a != b,
                _ => throw new System.Exception(),
            };
        }

        public override object Task2()
        {
            return highestEver;
        }
    }
}
