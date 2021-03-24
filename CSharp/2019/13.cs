using AdventOfCode;
using System;

namespace Advent2019
{
    public class Challenge13 : Challenge
    {
        const int DISPLAY_W = 40;
        const int DISPLAY_H = 24;

        char[] tiles = new char[] { ' ', '+', '#', '=', '@' };

        char[,] display = new char[DISPLAY_H, DISPLAY_W];
        public override object Task1()
        {
            IntCode.Emulator ICE = new IntCode.Emulator(input[0]);
            var response = IntCode.Emulator.ResultTemplate;

            int blockTiles = 0;

            while (true)
            {
                response = ICE.Run();
                if (response.Item1 == IntCode.ExitCode.Complete) break;
                long x = response.Item2;
                long y = ICE.Run().Item2;
                long tile = ICE.Run().Item2;
                display[y, x] = tiles[tile];
                if (tile == 2) blockTiles++;
            }
            return blockTiles;
        }

        public override object Task2()
        {
            long[] code = IntCode.Tools.ParseCode(input[0]);
            code[0] = 2;
            IntCode.Emulator ICE = new IntCode.Emulator(code);
            var response = IntCode.Emulator.ResultTemplate;

            long ballX = 0;
            long paddleX = 0;

            // make grid
            for (int _ = 0; _ < DISPLAY_H * DISPLAY_W; _++)
            {
                long x = ICE.Run().Item2;
                long y = ICE.Run().Item2;
                long tile = ICE.Run().Item2;
                if (tile == 4) ballX = x;
                if (tile == 3) paddleX = x;
            }

            // play
            while (true)
            {
                response = ICE.Run();
                if (response.Item1 == IntCode.ExitCode.Complete) return response.Item2;
                if (response.Item1 == IntCode.ExitCode.InputRequest)
                {
                    ICE.QueueInput(comp(ballX, paddleX));
                    ICE.Run();
                }
                long x = response.Item2;
                long y = ICE.Run().Item2;
                long z = ICE.Run().Item2;
                if (x == -1) continue;
                if (z == 4) ballX = x;
                if (z == 3) paddleX = x;
            }
        }

        private long comp(long a, long b)
        {
            if (a == b) return 0;
            if (a < b) return -1;
            return 1;
        }
    }
}