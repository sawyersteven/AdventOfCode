using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge17 : Challenge
    {
        private const char scaffold = '#';
        private const char empty = '.';


        private List<List<char>> grid;
        public override object Task1()
        {
            IntCode.Emulator ICE = new IntCode.Emulator();
            var response = ICE.Boot(IntCode.Tools.ParseCode(input[0]));
            ICE.ExpandMem(1800);

            grid = new List<List<char>>();

            List<char> row = new List<char>();
            while (true)
            {
                response = ICE.Run();
                if (response.Item2 == 10)
                {
                    if (row.Count == 0) break;
                    grid.Add(row);
                    row = new List<char>();
                    continue;
                }
                row.Add((char)response.Item2);
            }

            int alignParamSum = 0;
            for (int r = 1; r < grid.Count - 1; r++)
            {
                for (int c = 1; c < grid[r].Count - 1; c++)
                {
                    if (grid[r][c] == scaffold && grid[r + 1][c] + grid[r - 1][c] + grid[r][c + 1] + grid[r][c - 1] == scaffold * 4)
                    {
                        alignParamSum += c * r;
                    }
                }
            }
            return alignParamSum;
        }

        private const char NL = (char)10;
        public override object Task2()
        {
            IntCode.Emulator ICE = new IntCode.Emulator();
            long[] code = IntCode.Tools.ParseCode(input[0]);
            code[0] = 2;
            var response = ICE.Boot(code);
            ICE.ExpandMem(1800);

            // Complete program:
            // L,10,L,12,R,6 | R,10,L,4,L,4,L,12 | L,10,L,12,R,6 | R,10,L,4,L,4,L,12 | L,10,L,12,R,6 | L,10,R,10,R,6,L,4 | R,10,L,4,L,4,L,12 | L,10,R,10,R,6,L,4 | L,10,L,12,R,6 | L,10,R,10,R,6,L,4
            // A                                   A                                   A                                                                           A
            // L,10,L,12,R,6
            //                 B                                   B                                                       B
            //                 R,10,L,4,L,4,L,12
            //                                                                                         C                                       C                                   C        
            //                                                                                         L,10,R,10,R,6,L,4
            // A,B,A,B,A,C,B,C,A,C

            string Main = "A,B,A,B,A,C,B,C,A,C";
            string A = "L,10,L,12,R,6";
            string B = "R,10,L,4,L,4,L,12";
            string C = "L,10,R,10,R,6,L,4";

            Queue<char> outputbuffer = new Queue<char>();

            WaitForNextPrompt(ICE);

            Send(ICE, Main);
            WaitForNextPrompt(ICE);
            Send(ICE, A);
            WaitForNextPrompt(ICE);
            Send(ICE, B);
            WaitForNextPrompt(ICE);
            Send(ICE, C);
            WaitForNextPrompt(ICE);
            Send(ICE, "y");

            while (response.Item1 != IntCode.ExitCode.Complete)
            {
                response = ICE.Run();
            }

            return response.Item2;
        }

        private void WaitForNextPrompt(IntCode.Emulator ICE)
        {
            char last = ' ';
            while (last != ':' && last != '?')
            {
                last = (char)ICE.Run().Item2;
            }
            ICE.Run();
        }

        private long Send(IntCode.Emulator ICE, string instructions)
        {
            long[] inst = new long[instructions.Length + 1];
            for (int i = 0; i < instructions.Length; i++)
            {
                inst[i] = instructions[i];
            }
            inst[inst.Length - 1] = (char)10;
            return ICE.Run(inst).Item2;
        }
    }
}