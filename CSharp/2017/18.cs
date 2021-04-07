using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge18 : Challenge
    {
        public override object Task1()
        {
            Duet d = new Duet(0, input);
            long lastSent = 0;
            d.OnSend = (x) => lastSent = x;
            d.RunCode();
            return lastSent;
        }

        private class Duet
        {
            public Queue<long> InputQueue = new Queue<long>();
            private long LineNum = 0;
            private string[] prog;

            private long[] registers = new long[26];
            public Duet(int id, string[] program)
            {
                prog = program;
                registers['p' - 'a'] = id;
            }

            public Action<long> OnSend;

            private long GetValue(string instruction)
            {
                if (instruction[0] >= 'a' && instruction[0] <= 'z') return registers[instruction[0] - 'a'];
                else return long.Parse(instruction);
            }

            private ref long Register(string instruction)
            {
                return ref registers[instruction[0] - 'a'];
            }

            public bool RunCode()
            {
                for (; LineNum < prog.Length; LineNum++)
                {
                    string line = prog[LineNum];
                    string[] parts = prog[LineNum].Split(' ');


                    switch (prog[LineNum][1])
                    {
                        case 'n': // snd
                            OnSend(GetValue(parts[1]));
                            break;
                        case 'e': // set
                            Register(parts[1]) = GetValue(parts[2]);
                            break;
                        case 'd': // add
                            Register(parts[1]) += GetValue(parts[2]);
                            break;
                        case 'u': // mul
                            Register(parts[1]) *= GetValue(parts[2]);
                            break;
                        case 'o': // mod
                            Register(parts[1]) %= GetValue(parts[2]);
                            break;
                        case 'c': // rcv
                            if (InputQueue.Count == 0) return false;
                            Register(parts[1]) = InputQueue.Dequeue();
                            break;
                        case 'g': // jgz
                            if (GetValue(parts[1]) <= 0) break;
                            LineNum += GetValue(parts[2]) - 1;
                            break;
                    }
                }
                return true;
            }

        }

        public override object Task2()
        {
            Duet D0 = new Duet(0, input);
            Duet D1 = new Duet(1, input);
            int d1SendCount = 0;

            D0.OnSend = (x) =>
            {
                D1.InputQueue.Enqueue(x);
            };
            D1.OnSend = (x) =>
            {
                D0.InputQueue.Enqueue(x);
                d1SendCount++;
            };

            while (true)
            {
                bool d0complete = D0.RunCode();
                bool d1complete = D1.RunCode();

                if (d1complete && d0complete)
                {
                    return d1SendCount;
                }
                if (!d1complete && !d0complete && D1.InputQueue.Count == 0 && D0.InputQueue.Count == 0)
                {
                    return d1SendCount;
                }
            }
        }
    }
}
