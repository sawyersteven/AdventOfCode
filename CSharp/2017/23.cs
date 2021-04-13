using System;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge23 : Challenge
    {
        public override object Task1()
        {
            return RunCode(input);
        }

        private long GetValue(string instruction, long[] registers)
        {
            if (instruction[0] >= 'a' && instruction[0] <= 'z') return registers[instruction[0] - 'a'];
            else return long.Parse(instruction);
        }

        private ref long Register(string instruction, long[] registers)
        {
            return ref registers[instruction[0] - 'a'];
        }

        public int RunCode(string[] prog)
        {
            int mulCount = 0;
            long[] registers = new long[26];
            for (long LineNum = 0; LineNum < prog.Length; LineNum++)
            {
                string line = prog[LineNum];
                string[] parts = prog[LineNum].Split(' ');

                switch (prog[LineNum][2])
                {
                    case 't': // set
                        Register(parts[1], registers) = GetValue(parts[2], registers);
                        break;
                    case 'b': // sub
                        Register(parts[1], registers) -= GetValue(parts[2], registers);
                        break;
                    case 'l': // mul
                        mulCount++;
                        Register(parts[1], registers) *= GetValue(parts[2], registers);
                        break;
                    case 'z': // jnz
                        if (GetValue(parts[1], registers) == 0) break;
                        LineNum += GetValue(parts[2], registers) - 1;
                        break;
                    default:
                        throw new System.Exception();
                }
            }
            return mulCount;
        }

        public override object Task2()
        {
            var TC = new TranslatedCode();
            return TC.RunOptimized(1);
        }

        /* tl;dr:
                                          1000
        How many non-prime numbers are in ∑ 106500+n*17
                                          n=0
        */
        public class TranslatedCode
        {
            public int RunOptimized(int a)
            {
                int b, c, h = 0;
                if (a == 0)
                {
                    b = 65;
                    c = 65;
                }
                else
                {
                    b = 106500;
                    c = b + 17000;
                }

                for (; b <= c; b += 17)
                {
                    h += IsPrime(b) ? 0 : 1;
                }
                return h;
            }

            public bool IsPrime(int number)
            {
                if (number % 2 == 0) return false;
                for (int i = 3; i < number; i += 2)
                {
                    if (number % i == 0) return false;
                }

                return true;
            }

            public int RunRaw(int a)
            {
                if (a != 0)
                {
                    Console.WriteLine("You probably don't want to do this");
                    return -1;
                }

                int b, c, d, e, f, g, h;
                h = 0;

                // Pseudo-assembly starts here
                b = 65;
                c = b;
                if (a != 0) goto ln4;
                goto ln8;
            ln4:
                b *= 100;
                b += 100000;
                c = b;
                c += 17000;
            ln8:
                f = 1;
                d = 2;
            ln10:
                e = 2;
            ln11:
                g = d;
                g *= e;
                g -= b;
                if (g != 0) goto ln16;
                f = 0;
            ln16:
                e++;
                g = e;
                g -= b;
                if (g != 0) goto ln11;
                d++;
                g = d;
                g -= b;
                if (g != 0) goto ln10;
                if (f != 0) goto ln26;
                h++;
            ln26:
                g = b;
                g -= c;
                if (g != 0) goto ln30;
                goto ln32;
            ln30:
                b += 17;
                goto ln8;
            ln32:
                return h;
            }
        }
    }
}