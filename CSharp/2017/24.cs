using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge24 : Challenge
    {
        private struct Node
        {
            public readonly int A;
            public readonly int B;
            public bool Reversed;
            public HashSet<int> UsedIDs;
            public int Strength;
            public readonly int ID;

            public Node(int a, int b)
            {
                A = a;
                B = b;
                Reversed = false;
                UsedIDs = new HashSet<int>();
                Strength = 0;
                ID = A + (B << 16);
            }
        }

        List<Node> nodes = new List<Node>();
        public override void ParseInput()
        {
            foreach (string line in input)
            {
                int[] ab = Array.ConvertAll(line.Split('/'), int.Parse);
                nodes.Add(new Node(ab[0], ab[1]));
            }
        }

        private int strongestLongest = 0;
        /* All of this runs in Task1() and is probably a lot slower than it
        should be.
        */
        public override object Task1()
        {
            int bestStr = 0;

            int longest = 0;

            Stack<Node> s = new Stack<Node>();
            s.Push(new Node(0, 0));

            while (s.Count > 0)
            {
                Node current = s.Pop();
                if (current.Strength > bestStr) bestStr = current.Strength;
                if (current.UsedIDs.Count > longest)
                {
                    longest = current.UsedIDs.Count;
                    strongestLongest = current.Strength;
                }
                else if (current.UsedIDs.Count == longest)
                {
                    if (current.Strength > strongestLongest) strongestLongest = current.Strength;
                }

                int connectionValue = current.Reversed ? current.A : current.B;

                foreach (Node n in nodes)
                {
                    if (current.UsedIDs.Contains(n.ID) || current.ID == n.ID) continue;

                    if (n.A == connectionValue)
                    {
                        Node next = new Node(n.A, n.B);
                        next.UsedIDs.UnionWith(current.UsedIDs);
                        next.UsedIDs.Add(current.ID);
                        next.Strength = current.Strength + next.A + next.B;
                        s.Push(next);
                    }
                    else if (n.B == connectionValue)
                    {
                        Node next = new Node(n.A, n.B);
                        next.Reversed = true;
                        next.UsedIDs.UnionWith(current.UsedIDs);
                        next.UsedIDs.Add(current.ID);
                        next.Strength = current.Strength + next.A + next.B;
                        s.Push(next);
                    }
                }
            }

            return bestStr;
        }

        public override object Task2()
        {
            return strongestLongest;
        }
    }
}
