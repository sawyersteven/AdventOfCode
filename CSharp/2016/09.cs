using System;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge09 : Challenge
    {
        public override object Task1()
        {
            int totalLen = rawInput.Length;
            for (int i = 0; i < input.Length; i++)
            {
                if (rawInput[i] != '(') continue;

                int markerLen = rawInput.IndexOf(')', i) - i + 1;
                int[] markerParts = Array.ConvertAll(rawInput.Substring(i + 1, markerLen - 2).Split('x'), int.Parse);

                totalLen -= markerLen;
                i += markerParts[0];
                totalLen += markerParts[0] * (markerParts[1] - 1);

            }
            return totalLen;
        }

        public override object Task2()
        {
            return RecurseDecompress(rawInput);
        }

        private long RecurseDecompress(string input)
        {
            long len = input.Length;

            for (int i = 0; i < input.Length; i++)
            {
                if (input[i] != '(') continue;
                int markerLen = input.IndexOf(')', i) - i + 1;

                int[] markerParts = Array.ConvertAll(input.Substring(i + 1, markerLen - 2).Split('x'), int.Parse);

                string chunk = input.Substring(i + markerLen, markerParts[0]);

                long decompressedLen = RecurseDecompress(chunk);

                len += (decompressedLen * markerParts[1]) - markerParts[0] - markerLen;
                i += markerLen + markerParts[0] - 1;
            }

            return len;
        }
    }
}




