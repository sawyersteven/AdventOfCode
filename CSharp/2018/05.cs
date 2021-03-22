using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge05 : Challenge
    {

        public override object Task1()
        {
            return ReducePolymer(input[0]).Length;
        }

        private string ReducePolymer(string poly)
        {
            Stack<char> keep = new Stack<char>();
            foreach (char c in poly)
            {
                if (keep.Count == 0)
                {
                    keep.Push(c);
                }
                else if (c - 32 == keep.Peek() || c + 32 == keep.Peek())
                {
                    keep.Pop();
                }
                else
                {
                    keep.Push(c);
                }
            }
            return string.Join("", keep);
        }

        public override object Task2()
        {
            int shortest = int.MaxValue;

            for (char i = (char)65; i < 91; i++)
            {
                int len = ReducePolymer(StripLetter(input[0], i)).Length;
                if (len < shortest) shortest = len;
            }
            return shortest;
        }

        private string StripLetter(string s, char c)
        {
            Queue<char> q = new Queue<char>();
            for (int i = s.Length - 1; i > -1; i--)
            {
                if (s[i] != c && s[i] != c + 32)
                {
                    q.Enqueue(s[i]);
                }
            }
            return string.Join("", q);
        }
    }
}