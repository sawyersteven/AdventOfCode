using System;
using AdventOfCode;

namespace Advent2019
{
    public class Challenge19 : Challenge
    {
        private const int gridSize = 50;

        public override object Task1()
        {
            IntCode.Emulator ICE = new IntCode.Emulator(input[0]) { Verbose = false };

            long counter = 0;
            for (int y = 0; y < gridSize; y++)
            {
                for (int x = 0; x < gridSize; x++)
                {
                    ICE.QueueInput(x, y);
                    counter += ICE.Run().Item2;
                    ICE.Reboot();
                }
            }
            return counter;
        }

        public override object Task2()
        {
            return null;
        }
    }
}