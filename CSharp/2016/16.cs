using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2016
{
    public class Challenge16 : Challenge
    {
        private const int diskLen1 = 272;
        private const int diskLen2 = 35651584;

        public override object Task1()
        {
            bool[] src = ParseInput();
            src = FillDisk(src, diskLen1);
            src = Reduce(src);

            return ToBinString(src);
        }

        private string ToBinString(IList<bool> src)
        {
            char[] checksum = new char[src.Count];
            for (int i = 0; i < src.Count; i++)
            {
                checksum[i] = src[i] ? '1' : '0';
            }

            return string.Join(null, checksum);
        }

        private bool[] FillDisk(bool[] src, int size)
        {
            while (src.Length < size)
            {
                bool[] doubled = new bool[src.Length * 2 + 1];
                Array.Copy(src, doubled, src.Length);

                for (int i = 0; i < src.Length; i++)
                {
                    doubled[doubled.Length - (i + 1)] = !src[i];
                }
                src = doubled;
            }
            return new ArraySegment<bool>(src, 0, size).ToArray();
        }

        private bool[] Reduce(bool[] src)
        {
            while (src.Length % 2 == 0)
            {
                bool[] r = new bool[src.Length / 2];
                for (int ri = 0; ri < r.Length; ri++)
                {
                    r[ri] = (src[ri * 2] == src[ri * 2 + 1]);
                }
                src = r;
            }
            return src;
        }

        private bool[] ParseInput()
        {
            bool[] buffer = new bool[rawInput.Length];
            for (int i = 0; i < rawInput.Length; i++)
            {
                buffer[i] = (rawInput[i] == '0' ? false : true);
            }
            return buffer;
        }

        public override object Task2()
        {
            bool[] src = ParseInput();
            src = FillDisk(src, diskLen2);
            src = Reduce(src);

            return ToBinString(src);
        }
    }
}
