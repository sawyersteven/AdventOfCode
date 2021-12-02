using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2020
{

    public class Challenge23 : Challenge
    {
        private int[] inputNums;
        public override void ParseInput()
        {
            inputNums = new int[input[0].Length];
            for (int i = 0; i < inputNums.Length; i++)
            {
                inputNums[i] = (int)char.GetNumericValue(input[0][i]);
            }
        }

        #region T1
        private void RotateQueueTo<T>(Queue<T> q, T first)
        {
            T head = q.Peek();
            while (!head.Equals(first))
            {
                q.Enqueue(q.Dequeue());
                head = q.Peek();
            }
        }

        public override object Task1()
        {
            const int turns = 100;

            Queue<int> cups = new Queue<int>(inputNums);
            for (int _ = 0; _ < turns; _++)
            {
                int current = cups.Peek();

                // Console.WriteLine($"-- move {currentTurn} --");
                // Console.WriteLine($"cups: {string.Join(' ', labels).Replace(labels.Peek().ToString(), $"({labels.Peek().ToString()})")}");

                cups.Enqueue(cups.Dequeue());
                int[] pickedUpCups = new int[3];
                for (int i = 0; i < 3; i++)
                {
                    pickedUpCups[i] = cups.Dequeue();
                }
                // Console.WriteLine($"pick up: {string.Join(',', pickedUpCups)}");

                int destination = current - 1;
                if (destination == 0) destination = 9;
                while (Array.IndexOf(pickedUpCups, destination) != -1)
                {
                    destination--;
                    if (destination == 0) destination = 9;
                }

                // Console.WriteLine($"destination: {destination}");

                RotateQueueTo(cups, destination);
                cups.Enqueue(cups.Dequeue());

                foreach (int i in pickedUpCups)
                {
                    cups.Enqueue(i);
                }

                RotateQueueTo(cups, current);
                cups.Enqueue(cups.Dequeue());
            }

            RotateQueueTo(cups, 1);
            cups.Dequeue();

            return string.Join("", cups);
        }
        #endregion

        public override object Task2()
        {
            const int turns = 10000000;
            const int maxVal = 1000000;

            Dictionary<int, CircularLinkedListNode<int>> lookupTable = new Dictionary<int, CircularLinkedListNode<int>>();

            CircularLinkedList<int> cups = new CircularLinkedList<int>();
            foreach (int i in inputNums)
            {
                cups.AddLast(i);
                lookupTable[cups.Last.Value] = cups.Last;
            }

            for (int i = 10; i <= maxVal; i++)
            {
                cups.AddLast(i);
                lookupTable[cups.Last.Value] = cups.Last;
            }

            CircularLinkedListNode<int> current = cups.First;
            CircularLinkedListNode<int>[] pickedUpCups = new CircularLinkedListNode<int>[3];
            for (int _ = 0; _ < turns; _++)
            {
                for (int i = 0; i < pickedUpCups.Length; i++)
                {
                    pickedUpCups[i] = current.Next;
                    cups.Remove(pickedUpCups[i]);
                }

                CircularLinkedListNode<int> destNode = lookupTable[current.Value == 1 ? maxVal : current.Value - 1];

                while (Array.IndexOf(pickedUpCups, destNode) != -1)
                {
                    destNode = lookupTable[destNode.Value == 1 ? maxVal : destNode.Value - 1];
                }

                for (int i = pickedUpCups.Length - 1; i >= 0; i--)
                {
                    cups.AddAfter(destNode, pickedUpCups[i]);
                }

                current = current.Next;
            }

            CircularLinkedListNode<int> one = lookupTable[1];

            return (ulong)one.Next.Value * (ulong)one.Next.Next.Value == 5403610688;
        }
    }
}
