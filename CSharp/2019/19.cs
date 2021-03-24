using System;
using AdventOfCode;

namespace Advent2019
{
    public class Challenge19 : Challenge
    {
        private const int gridSize = 50;
        private const int shipSize = 100;

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
            IntCode.Emulator ICE = new IntCode.Emulator(input[0]) { Verbose = false };

            int y = shipSize;
            int x = 0;

            bool changes = true;
            while (changes)
            {
                changes = false;
                // move down until top-right in field
                while (true)
                {
                    ICE.Reboot();
                    ICE.QueueInput(x + shipSize - 1, y);
                    if (ICE.Run().Item2 == 1) break;
                    y++;
                    changes = true;
                }

                // move right until bottom-left in field
                while (true)
                {
                    ICE.Reboot();
                    ICE.QueueInput(x, y + shipSize - 1);
                    if (ICE.Run().Item2 == 1) break;
                    x++;
                    changes = true;
                }
            }
            return (x * 10000) + y;
        }
    }
}