using System;
using AdventOfCode;
using ExtensionMethods;

namespace Advent2016
{
    public class Challenge08 : Challenge
    {
        const int width = 50;
        const int height = 6;
        char[,] panel = new char[height, width];

        public override object Task1()
        {
            int[] vals;
            foreach (string line in input)
            {
                if (line.StartsWith("rect"))
                {
                    vals = Array.ConvertAll(line.Split(' ')[1].Split('x'), int.Parse);
                    Rect(vals[0], vals[1]);
                    continue;
                }
                vals = Array.ConvertAll(line.Split('=')[1].Split(" by "), int.Parse);

                if (line[7] == 'r') RotateRow(vals[0], vals[1]);
                else RotateCol(vals[0], vals[1]);
            }

            int pixels = 0;
            for (int y = 0; y < height; y++)
            {
                for (int x = 0; x < width; x++)
                {
                    if (panel[y, x] == '#') pixels++;
                }
            }
            return pixels;
        }

        private void Rect(int w, int h)
        {
            for (int y = 0; y < h; y++)
            {
                for (int x = 0; x < w; x++)
                {
                    panel[y, x] = '#';
                }
            }
        }

        private void RotateCol(int col, int by)
        {
            int h = panel.GetLength(0);
            for (int _ = 0; _ < by; _++)
            {
                char last = panel[h - 1, col];
                for (int y = h - 1; y > 0; y--)
                {
                    panel[y, col] = panel[y - 1, col];
                }
                panel[0, col] = last;
            }
        }

        private void RotateRow(int row, int by)
        {
            int w = panel.GetLength(1);
            for (int _ = 0; _ < by; _++)
            {
                char last = panel[row, w - 1];
                for (int x = w - 1; x > 0; x--)
                {
                    panel[row, x] = panel[row, x - 1];
                }
                panel[row, 0] = last;
            }
        }

        public override object Task2()
        {
            return '\n' + panel.Render();
        }
    }
}
