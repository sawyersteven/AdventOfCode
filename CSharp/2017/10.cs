using System;
using System.Text;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge10 : Challenge
    {
        public class KnotHash
        {
            public readonly byte[] DenseHash;
            public string HashCode { get; private set; }

            private static readonly byte[] suffix = new byte[] { 17, 31, 73, 47, 23 };

            public KnotHash(string input)
            {
                byte[] byteString = Encoding.ASCII.GetBytes(input);
                byte[] lengths = new byte[byteString.Length + suffix.Length];
                Array.Copy(byteString, lengths, input.Length);
                Array.Copy(suffix, 0, lengths, byteString.Length, suffix.Length);

                CircularLinkedList<byte> circle = new CircularLinkedList<byte>();
                for (int i = 0; i <= byte.MaxValue; i++) circle.AddLast((byte)i);

                TieKnot(circle, lengths, 64);

                DenseHash = MakeDenseHash(circle.ToArray());
                HashCode = MakeHashCode(DenseHash);
            }

            public override string ToString() => HashCode;

            private string MakeHashCode(byte[] denseHash)
            {
                string[] hash = new string[denseHash.Length];
                for (int i = 0; i < denseHash.Length; i++)
                {
                    hash[i] = ((int)denseHash[i]).ToString("x2");
                }
                return string.Join("", hash);
            }

            private byte[] MakeDenseHash(byte[] sparse)
            {
                byte[] dense = new byte[16];
                for (int i = 0; i < 16; i++)
                {
                    byte xord = 0;

                    for (int j = (i * 16); j < (i + 1) * 16; j++)
                    {
                        xord ^= sparse[j];
                    }
                    dense[i] = xord;
                }
                return dense;
            }

            private void TieKnot(CircularLinkedList<byte> circle, byte[] lengths, int iterations = 1)
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
        }

        public override object Task1()
        {
            CircularLinkedList<byte> circle = new CircularLinkedList<byte>();
            for (int i = 0; i <= byte.MaxValue; i++)
            {
                circle.AddLast((byte)i);
            }

            string[] parts = rawInput.Split(',');
            byte[] lengths = new byte[parts.Length];
            for (int i = 0; i < parts.Length; i++)
            {
                lengths[i] = byte.Parse(parts[i]);
            }
            int totalSkips = 0;
            int skipSize = 0;

            foreach (int l in lengths)
            {
                ReverseSegment(circle, l);
                circle.MoveHeadRight(skipSize + l);
                totalSkips += skipSize + l;
                skipSize++;
            }
            circle.MoveHeadLeft(totalSkips);

            return circle.First.Value * circle.First.Next.Value;
        }


        private static void ReverseSegment(CircularLinkedList<byte> arr, int len)
        {
            CircularLinkedListNode<byte> start = arr.First;
            CircularLinkedListNode<byte> end = start;
            for (int i = 0; i < len - 1; i++) end = end.Next;

            for (int _ = 0; _ < len / 2; _++)
            {
                start.Value += end.Value;
                end.Value = (byte)(start.Value - end.Value);
                start.Value -= end.Value;

                start = start.Next;
                end = end.Previous;
            }
        }

        public override object Task2()
        {
            return new KnotHash(rawInput).HashCode;
        }
    }
}
