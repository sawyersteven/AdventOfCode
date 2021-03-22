using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge18 : Challenge
    {

        private enum Op
        {
            Add,
            Multiply
        }

        private bool isTask2 = false;

        public override object Task1()
        {
            ulong total = 0;
            foreach (string line in input)
            {
                total += SolveLine(line);
            }
            return total;
        }

        private ulong SolveLine(string line)
        {
            while (true)
            {
                int open = -1;
                int close = -1;
                for (int i = 0; i < line.Length; i++)
                {
                    if (line[i] == '(') open = i;
                    if (line[i] == ')')
                    {
                        close = i;
                        break;
                    }
                }
                if (open == -1) return isTask2 ? SolveChunkT2(line) : SolveChunk(line);

                ulong result = isTask2 ? SolveChunkT2(line.Substring(open + 1, (close - open) - 1)) : SolveChunk(line.Substring(open + 1, (close - open) - 1));
                line = line.Remove(open, (close - open) + 1).Insert(open, result.ToString());
            }
        }

        private ulong SolveChunk(string chunk)
        {
            ulong result = 0;
            Op nextOp = Op.Add;

            string[] parts = chunk.Split(' ');
            ulong nextNum;
            foreach (string part in parts)
            {
                if (ulong.TryParse(part, out nextNum))
                {
                    if (nextOp == Op.Add) result += nextNum;
                    else if (nextOp == Op.Multiply) result *= nextNum;
                }
                else if (part == "*") nextOp = Op.Multiply;
                else if (part == "+") nextOp = Op.Add;
            }

            return result;
        }

        public override object Task2()
        {
            isTask2 = true;
            ulong total = 0;
            foreach (string line in input)
            {
                total += SolveLine(line);
            }
            return total;
        }

        private ulong SolveChunkT2(string chunk)
        {
            string[] parts = chunk.Split(' ');
            for (int i = 0; i < parts.Length; i++)
            {
                if (parts[i] == "+")
                {
                    ulong a = ulong.Parse(parts[i - 1]);
                    ulong b = ulong.Parse(parts[i + 1]);
                    parts[i - 1] = (a + b).ToString();
                    parts[i] = parts[i + 1] = "";

                    for (int j = i + 2; j < parts.Length; j++)
                    {
                        parts[j - 2] = parts[j];
                    }
                    parts[parts.Length - 1] = parts[parts.Length - 2] = "";
                    i -= 1;
                }
            }

            ulong result = ulong.Parse(parts[0]);

            for (int i = 1; i < parts.Length; i++)
            {
                if (parts[i] == "" || parts[i] == "*") continue;
                result *= ulong.Parse(parts[i]);
            }

            return result;
        }

    }
}