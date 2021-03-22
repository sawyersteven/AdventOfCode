using AdventOfCode;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge08 : Challenge
    {
        private struct Node
        {
            public int Offset;
            public int Length;
            public (int, int) Header;
            public Node[] Children;
            public int[] Metadata;
            public char ID;
        }

        char ID = 'A';
        private Node MakeNode(int offset)
        {
            Node n = new Node();
            n.ID = ID;
            n.Offset = offset;
            n.Header = (intInput[offset], intInput[offset + 1]);
            n.Children = new Node[n.Header.Item1];
            n.Metadata = new int[n.Header.Item2];
            n.Length = 2 + n.Header.Item2;
            ID++;
            return n;
        }

        private int[] intInput;
        private Node root;
        public override object Task1()
        {
            string[] parts = input[0].Split(' ');
            intInput = new int[parts.Length];
            for (int i = 0; i < parts.Length; i++)
            {
                intInput[i] = int.Parse(parts[i]);
            }

            root = MakeNode(0);
            MakeChildren(root);

            return CountMetadataValues(root);
        }

        private int CountMetadataValues(Node root)
        {
            int count = 0;
            foreach (int i in root.Metadata) count += i;
            foreach (Node child in root.Children)
            {
                count += CountMetadataValues(child);
            }
            return count;

        }

        private int MakeChildren(Node n)
        {
            int workingOffset = n.Offset + 2;
            for (int i = 0; i < n.Children.Length; i++)
            {
                Node child = MakeNode(workingOffset);
                n.Children[i] = child;
                int childLen = MakeChildren(child);
                n.Length += childLen;
                workingOffset += childLen;
            }
            for (int i = 0; i < n.Metadata.Length; i++)
            {
                n.Metadata[i] = intInput[n.Offset + n.Length - n.Metadata.Length + i];
            }
            return n.Length;
        }

        public override object Task2()
        {
            return CountNodeValues(root);
        }

        private int CountNodeValues(Node root)
        {
            int total = 0;
            if (root.Children.Length == 0)
            {
                foreach (int i in root.Metadata) total += i;
            }
            else
            {
                foreach (int i in root.Metadata)
                {
                    if (i - 1 < root.Children.Length)
                    {
                        total += CountNodeValues(root.Children[i - 1]);
                    }
                }
            }
            return total;
        }
    }
}