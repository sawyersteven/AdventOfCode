using System;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge10 : Challenge
    {
        public override object Task1()
        {
            CircularLinkedList<int> circle = new CircularLinkedList<int>();
            for (int i = 0; i < 256; i++) circle.AddLast(i);

            string[] parts = rawInput.Split(',');
            char[] lengths = new char[parts.Length];
            for (int i = 0; i < parts.Length; i++)
            {
                lengths[i] = (char)int.Parse(parts[i]);
            }

            TieKnot(circle, lengths);

            return circle.First.Value * circle.First.Next.Value;
        }

        private void TieKnot(CircularLinkedList<int> circle, char[] lengths, int iterations = 1)
        {
            int totalSkips = 0;
            int skipSize = 0;
            for (int i = 0; i < iterations; i++)
            {
                foreach (int l in lengths)
                {
                    ReverseSegment(circle, l);
                    circle.MoveHeadRight(skipSize + l);
                    totalSkips += skipSize + l;
                    skipSize++;
                }
            }
            circle.MoveHeadLeft(totalSkips);
        }

        private void ReverseSegment(CircularLinkedList<int> arr, int len)
        {
            CircularLinkedListNode<int> start = arr.First;
            CircularLinkedListNode<int> end = start;
            for (int i = 0; i < len - 1; i++) end = end.Next;

            for (int _ = 0; _ < len / 2; _++)
            {
                start.Value += end.Value;
                end.Value = start.Value - end.Value;
                start.Value -= end.Value;

                start = start.Next;
                end = end.Previous;
            }
        }

        private char[] suffix = new char[] { (char)17, (char)31, (char)73, (char)47, (char)23 };
        public override object Task2()
        {
            char[] byteString = rawInput.ToCharArray();
            char[] lengths = new char[rawInput.Length + suffix.Length];
            Array.Copy(rawInput.ToCharArray(), lengths, rawInput.Length);
            Array.Copy(suffix, 0, lengths, rawInput.Length, suffix.Length);

            CircularLinkedList<int> circle = new CircularLinkedList<int>();
            for (int i = 0; i < 256; i++) circle.AddLast(i);

            TieKnot(circle, lengths, 64);

            int[] dh = DenseHash(circle.ToArray());

            string[] hash = new string[dh.Length];
            for (int i = 0; i < dh.Length; i++)
            {
                hash[i] = dh[i].ToString("X2");
            }

            return string.Join("", hash).ToLower();
        }

        private int[] DenseHash(int[] sparse)
        {
            int[] dense = new int[16];
            for (int i = 0; i < dense.Length; i++)
            {
                int xord = 0;

                for (int j = (i * 16); j < (i + 1) * 16; j++)
                {
                    xord ^= sparse[j];
                }
                dense[i] = xord;
            }
            return dense;
        }
    }
}
