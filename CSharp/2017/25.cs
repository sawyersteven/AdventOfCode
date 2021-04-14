using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge25 : Challenge
    {
        Dictionary<int, int[][]> Rules = new Dictionary<int, int[][]>();

        private (int, int) ParseInput()
        {
            (int, int) start_iters = (0, 0);
            start_iters.Item1 = input[0][^2] - 'A';
            start_iters.Item2 = int.Parse(input[1].Split(' ')[^2]);

            for (int i = 0; i < input.Length; i++)
            {
                if (input[i] == string.Empty || input[i][0] != 'I') continue;
                int state = input[i][^2] - 'A';
                i++;
                int[][] commands = new int[2][];
                for (; i < input.Length; i++)
                {
                    if (input[i] == string.Empty) break;
                    int rulenum = (int)char.GetNumericValue(input[i][^2]);
                    int writeVal = input[i + 1][^2] == '0' ? 0 : 1;
                    int moveDir = input[i + 2][^3] == 'f' ? -1 : 1; // leFt.
                    int endState = input[i + 3][^2] - 'A';
                    commands[rulenum] = new int[] { writeVal, moveDir, endState };
                    i += 3;
                }
                Rules[state] = commands;
            }
            return start_iters;
        }

        public override object Task1()
        {
            (int state, int steps) = ParseInput();

            LinkedList<int> tape = new LinkedList<int>();
            tape.AddFirst(0);

            LinkedListNode<int> cursor = tape.First;

            for (int _ = 0; _ < steps; _++)
            {
                int[] commands = Rules[state][cursor.Value];
                cursor.Value = commands[0];

                if (commands[1] == 1)
                {
                    cursor = cursor.Next ?? tape.AddLast(0);
                }
                else
                {
                    cursor = cursor.Previous ?? tape.AddFirst(0);
                }
                state = commands[2];
            }

            int checksum = 0;
            var current = tape.First;
            while (current != null)
            {
                checksum += current.Value;
                current = current.Next;
            }
            return checksum;
        }

        public override object Task2()
        {
            return '*';
        }
    }
}
