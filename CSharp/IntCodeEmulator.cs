using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class IntCodeEmulator
    {
        public List<int> output = new List<int>();

        public void Run(string code, int c3Input = 3)
        {
            string[] parts = code.Split(',');
            int[] intCode = new int[parts.Length];
            for (int i = 0; i < intCode.Length; i++) intCode[i] = int.Parse(parts[i]);
            Run(intCode, c3Input);
        }

        public void Run(int[] prog, int c3Input = 0)
        {
            int position = 0;
            output.Clear();

            int Param1() => (prog[position] / 100 % 10) == 0 ? prog[prog[position + 1]] : prog[position + 1];
            int Param2() => (prog[position] / 1000 % 10) == 0 ? prog[prog[position + 2]] : prog[position + 2];
            void Write(int val) => prog[prog[position + 3]] = val;

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
                        output.Add(Param1());
                        position += 2;
                        break;
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
                        return;
                    default:
                        throw new Exception("Invalid OpCode: " + prog[position] % 100);
                }
            }
            throw new Exception("Intcode exited prematurely");
        }
    }
}