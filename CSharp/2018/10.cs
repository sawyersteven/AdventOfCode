using System.Text.RegularExpressions;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2018
{
    public class Challenge10 : Challenge
    {
        private const int LetterHeight = 10;

        private struct Star
        {
            public int PosX;
            public int PosY;
            public int VelX;
            public int VelY;
        }

        public override void ParseInput()
        {
            Regex rx = new Regex(@"[.*]?-?(\d+)", RegexOptions.Compiled);

            stars = new Star[input.Length];

            for (int i = 0; i < input.Length; i++)
            {
                MatchCollection matches = rx.Matches(input[i]);
                Star s = new Star();
                s.PosX = int.Parse(matches[0].Value);
                s.PosY = int.Parse(matches[1].Value);
                s.VelX = int.Parse(matches[2].Value);
                s.VelY = int.Parse(matches[3].Value);
                stars[i] = s;
            }
        }

        private Star[] stars;
        public override object Task1()
        {
            for (int seconds = 1; ; seconds++)
            {
                int minY = int.MaxValue;
                int maxY = 0;
                int minX = int.MaxValue;
                int maxX = 0;

                for (int i = 0; i < stars.Length; i++)
                {
                    stars[i].PosX += stars[i].VelX;
                    stars[i].PosY += stars[i].VelY;
                    if (stars[i].PosY > maxY) maxY = stars[i].PosY;
                    if (stars[i].PosY < minY) minY = stars[i].PosY;
                    if (stars[i].PosX > maxX) maxX = stars[i].PosX;
                    if (stars[i].PosX < minX) minX = stars[i].PosX;
                }

                if (maxY - minY + 1 == LetterHeight)
                {
                    char[,] grid = new char[maxY - minY + 1, maxX - minX + 1];

                    foreach (Star s in stars)
                    {
                        grid[s.PosY - minY, s.PosX - minX] = '#';
                    }
                    grid.Print();

                    return seconds;
                }
            }
        }

        public override object Task2()
        {
            return "See Task 1 answer";
        }
    }
}