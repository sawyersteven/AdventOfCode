using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge10 : Challenge
    {
        public override object Task1()
        {
            List<char> seq = new List<char>(rawInput.ToCharArray());

            for (int _ = 0; _ < 40; _++)
            {
                seq = SeeSay(seq);
            }
            return seq.Count;
        }

        private List<char> SeeSay(List<char> seq)
        {
            List<char> next = new List<char>();
            for (int i = 0; i < seq.Count; i++)
            {
                int charCount = 1;
                while (i < seq.Count - 1 && seq[i + 1] == seq[i])
                {
                    charCount++;
                    i++;
                }
                next.Add(charCount.ToString()[0]);
                next.Add(seq[i]);
            }
            return next;
        }

        public override object Task2()
        {
            List<char> seq = new List<char>(rawInput.ToCharArray());

            for (int _ = 0; _ < 50; _++)
            {
                seq = SeeSay(seq);
            }
            return seq.Count;
        }
    }
}
