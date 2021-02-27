using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace Advent2019
{
    public abstract class Challenge
    {
        public ChallengeResult result1;
        public ChallengeResult result2;

        protected List<string> input;

        public void Go(List<string> input)
        {
            this.input = input;

            Stopwatch sw = new Stopwatch();
            sw.Start();
            object r = Task1();
            sw.Stop();
            result1 = new ChallengeResult(r, sw.Elapsed.TotalMilliseconds);

            sw.Restart();
            r = Task2();
            sw.Stop();
            result2 = new ChallengeResult(r, sw.Elapsed.TotalMilliseconds);

        }

        public abstract object Task1();
        public abstract object Task2();
    }

    public class ChallengeResult
    {
        public double Time { get; private set; }
        public string Answer { get; private set; }

        public ChallengeResult(object answer, double time)
        {
            Time = time;
            Answer = answer?.ToString();
        }
    }

    public static class Tools
    {
        public static void Print2DGrid(char[,] grid)
        {
            int H = grid.GetLength(0);
            int W = grid.GetLength(1);
            for (int col = 0; col < H; col++)
            {
                char[] r = new char[W];
                for (int row = 0; row < W; row++)
                {
                    r[row] = grid[col, row];
                }
                Console.WriteLine(string.Join("", r));
            }
        }
    }
}