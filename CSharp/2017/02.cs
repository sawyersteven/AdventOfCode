using System;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge02 : Challenge
    {
        int[][] spreadSheet;
        public override object Task1()
        {
            spreadSheet = new int[input.Length][];
            for (int i = 0; i < input.Length; i++)
            {
                spreadSheet[i] = Array.ConvertAll(input[i].Split('\t'), int.Parse);
            }

            int sum = 0;
            foreach (int[] line in spreadSheet)
            {
                int min = int.MaxValue;
                int max = 0;
                foreach (int i in line)
                {
                    if (i > max) max = i;
                    if (i < min) min = i;
                }
                sum += max - min;
            }
            return sum;
        }

        public override object Task2()
        {
            int sum = 0;
            foreach (int[] line in spreadSheet)
            {
                bool lineDone = false;
                for (int i = 0; i < line.Length - 1; i++)
                {
                    for (int j = i + 1; j < line.Length; j++)
                    {
                        if (line[i] % line[j] == 0)
                        {
                            sum += line[i] / line[j];
                            lineDone = true;
                            break;
                        }
                        else if (line[j] % line[i] == 0)
                        {
                            sum += line[j] / line[i];
                            lineDone = true;
                            break;
                        }
                    }
                    if (lineDone) break;
                }

            }

            return sum;
        }
    }
}
