using System;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2015
{
    public class Challenge06 : Challenge
    {
        private const int on = 1;
        private const int off = 0;

        private int TurnOff(int _) => off;
        private int TurnOn(int _) => on;
        private int Toggle(int val) => val == on ? off : on;

        private int TurnDown(int val)
        {
            val -= 1;
            if (val < 0) val = 0;
            return val;
        }
        private int TurnUp(int val) => val + 1;
        private int TurnUp2(int val) => val + 2;

        public override object Task1()
        {
            int[,] grid = new int[1000, 1000];
            grid.Fill(off);

            Func<int, int> command;

            foreach (string line in input)
            {
                string[] parts = line.Split(' ');
                int[] start = Array.ConvertAll(parts[^3].Split(','), int.Parse);
                int[] end = Array.ConvertAll(parts[^1].Split(','), int.Parse);

                if (parts[1] == "off") command = TurnOff;
                else if (parts[1] == "on") command = TurnOn;
                else command = Toggle;

                for (int y = start[1]; y <= end[1]; y++)
                {
                    for (int x = start[0]; x <= end[0]; x++)
                    {
                        grid[y, x] = command(grid[y, x]);
                    }
                }
            }

            return grid.Count(on);
        }

        public override object Task2()
        {
            int[,] grid = new int[1000, 1000];
            grid.Fill(off);

            Func<int, int> command;

            foreach (string line in input)
            {
                string[] parts = line.Split(' ');
                int[] start = Array.ConvertAll(parts[^3].Split(','), int.Parse);
                int[] end = Array.ConvertAll(parts[^1].Split(','), int.Parse);

                if (parts[1] == "off") command = TurnDown;
                else if (parts[1] == "on") command = TurnUp;
                else command = TurnUp2;

                for (int y = start[1]; y <= end[1]; y++)
                {
                    for (int x = start[0]; x <= end[0]; x++)
                    {
                        grid[y, x] = command(grid[y, x]);
                    }
                }
            }

            long t = 0;
            foreach (int i in grid)
            {
                t += i;
            }
            return t;
        }
    }
}
