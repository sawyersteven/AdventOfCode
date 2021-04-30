using System;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge23 : Challenge
    {
        public override object Task1()
        {
            int[] reg = new int[2];
            Run(reg, input);
            return reg[1];
        }


        private void Run(int[] registers, string[] code)
        {
            for (int i = 0; i < code.Length; i++)
            {
                string line = code[i];
                switch (line[2])
                {
                    case 'f': // hlf
                        registers[line[4] - 'a'] /= 2;
                        break;
                    case 'l': // tpl
                        registers[line[4] - 'a'] *= 3;
                        break;
                    case 'c': // inc
                        registers[line[4] - 'a']++;
                        break;
                    case 'p': // jmp
                        i += int.Parse(line.Split(' ')[^1]) - 1;
                        break;
                    case 'e':  // jie
                        if (registers[line[4] - 'a'] % 2 == 0)
                        {
                            i += int.Parse(line.Split(' ')[^1]) - 1;
                        }
                        break;
                    case 'o': // jio
                        if (registers[line[4] - 'a'] == 1)
                        {
                            i += int.Parse(line.Split(' ')[^1]) - 1;
                        }
                        break;
                }
            }
        }

        public override object Task2()
        {
            return Translated();
        }

        private ulong Translated()
        {
            ulong a = 1;
            ulong b = 0;

            a = 113383;
            while (a != 1)
            {
                b++;
                if (a % 2 == 0)
                {
                    a /= 2;
                    continue;
                }
                a *= 3;
                a++;
            }
            return b;
        }
    }
}
