using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge10 : Challenge
    {
        private char GetPair(char c) => c switch
        {
            '(' => ')',
            ')' => '(',
            '[' => ']',
            ']' => '[',
            '{' => '}',
            '}' => '{',
            '<' => '>',
            '>' => '<',
            _ => throw new System.Exception(),
        };

        private bool IsOpen(char c) => c == '<' || c == '(' || c == '[' || c == '{';

        private List<int> incompleteLines;
        public override object Task1()
        {
            incompleteLines = new List<int>(input.Length / 2);
            uint score = 0;

            Stack<char> stack = new Stack<char>();
            for (int i = 0; i < input.Length; i++)
            {
                stack.Clear();
                bool incomplete = true;
                foreach (char c in input[i])
                {
                    if (IsOpen(c)) stack.Push(c);
                    else if (stack.Peek() == GetPair(c)) stack.Pop();
                    else
                    {
                        score += c switch
                        {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => throw new System.Exception(),
                        };
                        incomplete = false;
                        break;
                    }
                }
                if (incomplete) incompleteLines.Add(i);
            }
            return score;
        }

        public override object Task2()
        {
            Stack<char> stack = new Stack<char>();
            long[] scores = new long[incompleteLines.Count];
            for (int i = 0; i < incompleteLines.Count; i++)
            {
                stack.Clear();
                foreach (char c in input[incompleteLines[i]])
                {
                    if (IsOpen(c)) stack.Push(c);
                    else if (stack.Peek() == GetPair(c)) stack.Pop();
                }
                char[] complete = new char[stack.Count];
                for (int j = 0; j < complete.Length; j++)
                {
                    complete[j] = GetPair(stack.Pop());
                }
                scores[i] = Score(complete);
            }

            Array.Sort(scores);
            return scores[scores.Length / 2];
        }

        private long Score(char[] complete)
        {
            long score = 0;
            foreach (char c in complete)
            {
                score *= 5;
                switch (c)
                {
                    case ')': score += 1; break;
                    case ']': score += 2; break;
                    case '}': score += 3; break;
                    case '>': score += 4; break;
                }
            }
            return score;
        }
    }
}
