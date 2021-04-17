using System;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge09 : Challenge
    {
        public override object Task1()
        {
            return null;
            return rawInput.Length + DecompressedLen(rawInput);
        }
        public long DecompressedLen(string data, bool pt1 = true)
        {
            long addedChars = 0;
            for (int i = 0; i < data.Length; i++)
            {
                if (data[i] == '(')
                {
                    int markerLen = data.Substring(i).IndexOf(')') + 1;
                    int[] marker = Array.ConvertAll(data.Substring(i + 1, markerLen - 2).Split('x'), int.Parse);
                    addedChars -= markerLen;
                    if (pt1) i += marker[0];
                    else
                    {
                        string decompressedChunk = string.Join(data.Substring(i + markerLen, marker[0]), new string[marker[1] + 1]);
                        addedChars += DecompressedLen(decompressedChunk, false);
                        i += marker[0];
                    }
                    addedChars += marker[0] * (marker[1] - 1);
                }
            }
            return addedChars;
        }
        public override object Task2()
        {
            return rawInput.Length + DecompressedLen(rawInput, false);
        }
    }
}




