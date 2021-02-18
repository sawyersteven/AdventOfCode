using System;
using System.Collections.Generic;

namespace Advent2019
{
    namespace IntCode
    {
        public enum ExitCode
        {
            EOF = 38,
            OutputDelivery = 4,
            Complete = 99,
            InvalidCommand = 1,
            Null = 0
        }

        public static class Tools
        {
            public static int[] ParseCode(string code)
            {
                string[] parts = code.Split(',');
                int[] intCode = new int[parts.Length];
                for (int i = 0; i < intCode.Length; i++) intCode[i] = int.Parse(parts[i]);
                return intCode;
            }
        }

        public class Emulator
        {
            private int[] prog;
            private int position = 0;

            public Emulator(int[] program)
            {
                prog = program;
            }

            public void Reboot(int[] program)
            {
                position = 0;
                prog = program;
            }

            int Param1() => (prog[position] / 100 % 10) == 0 ? prog[prog[position + 1]] : prog[position + 1];
            int Param2() => (prog[position] / 1000 % 10) == 0 ? prog[prog[position + 2]] : prog[position + 2];
            void Write(int val) => prog[prog[position + 3]] = val;

            public (ExitCode, int) Run(int c3Input = 0)
            {
                while (position < prog.Length)
                {
                    int opCode = prog[position] % 100;
                    switch (opCode)
                    {
                        case 1:
                            Write(Param1() + Param2());
                            position += 4;
                            break;
                        case 2:
                            Write(Param1() * Param2());
                            position += 4;
                            break;
                        case 3:
                            prog[prog[position + 1]] = c3Input;
                            position += 2;
                            break;
                        case 4:
                            int ret = Param1();
                            position += 2;
                            return (ExitCode.OutputDelivery, ret);
                        case 5:
                            if (Param1() != 0) position = Param2();
                            else position += 3;
                            break;
                        case 6:
                            if (Param1() == 0) position = Param2();
                            else position += 3;
                            break;
                        case 7:
                            Write((Param1() < Param2()) ? 1 : 0);
                            position += 4;
                            break;
                        case 8:
                            Write((Param1() == Param2()) ? 1 : 0);
                            position += 4;
                            break;
                        case 99:
                            return (ExitCode.Complete, 0);
                        default:
                            return (ExitCode.InvalidCommand, prog[position] % 100); // invalid opcode
                    }
                }
                return (ExitCode.EOF, 0); // Exit prematurely
            }
        }
    }
}