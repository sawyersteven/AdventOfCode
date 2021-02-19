using System;
using System.Collections.Generic;
using System.Text;

namespace Advent2019
{
    public class Challenge08 : Challenge
    {
        const int W = 25;
        const int H = 6;
        const int layerSize = 25 * 6;
        const char black = '0';
        const char white = '1';
        const char transparent = '2';

        public override object Task1()
        {
            (int, int, int) matchingLayer = (int.MaxValue, 0, 0);

            for (int i = 0; i < input[0].Length; i += layerSize)
            {
                (int, int, int) counts = (0, 0, 0);
                foreach (char c in input[0].Substring(i, layerSize))
                {
                    if (c == '0') counts.Item1++;
                    else if (c == '1') counts.Item2++;
                    else if (c == '2') counts.Item3++;
                }
                if (counts.Item1 < matchingLayer.Item1) matchingLayer = counts;
            }

            return matchingLayer.Item2 * matchingLayer.Item3;
        }

        public override object Task2()
        {
            List<string[]> layers = new List<string[]>();

            for (int i = 0; i < input[0].Length; i += layerSize)
            {
                string[] layer = new string[H];
                string l = input[0].Substring(i, layerSize);
                for (int j = 0; j * W < l.Length; j++)
                {
                    layer[j] = l.Substring(j * W, W);
                }
                layers.Add(layer);
            }

            char[][] image = new char[H][];

            for (int h = 0; h < H; h++)
            {
                image[h] = new char[W];
                for (int w = 0; w < W; w++)
                {
                    foreach (string[] layer in layers)
                    {
                        if (layer[h][w] == transparent) continue;
                        if (layer[h][w] == black) image[h][w] = ' ';
                        else image[h][w] = '#';
                        break;
                    }
                }
            }

            StringBuilder sb = new StringBuilder(H);
            foreach (char[] row in image)
            {
                sb.AppendLine(string.Join("", row));
            }

            Console.WriteLine(sb.ToString());

            return null;
        }
    }
}