using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge19 : Challenge
    {
        private string[] Rules;
        private List<string> Messages;

        private void ParseRules()
        {
            Rules = new ArraySegment<string>(input, 0, Array.IndexOf(input, string.Empty)).ToArray();

            Array.Sort(Rules, (a, b) => int.Parse(a.Split(":")[0]).CompareTo(int.Parse(b.Split(":")[0])));
            for (int i = 0; i < Rules.Length; i++)
            {
                Rules[i] = Rules[i].Split(": ")[1].Replace("\"", "");
            }
        }

        private void ParseMessages()
        {
            int start = Array.IndexOf(input, string.Empty) + 1;
            Messages = new List<string>(new ArraySegment<string>(input, start, input.Length - start));
        }

        public override object Task1()
        {
            ParseRules();
            ParseMessages();

            return FindMatches(0);
        }

        private ulong FindMatches(int RuleNum)
        {
            int maxMessageLength = 0;
            foreach (string m in Messages)
            {
                if (m.Length > maxMessageLength) maxMessageLength = m.Length;
            }

            Queue<string> possibilities = new Queue<string>();
            possibilities.Enqueue(Rules[RuleNum]);

            ulong matches = 0;
            while (possibilities.Count > 0)
            {
                bool checkMessages = true;

                string code = possibilities.Dequeue();

                string[] codeParts = code.Split(' ');

                for (int i = 0; i < codeParts.Length; i++)
                {
                    if (codeParts[i] == "a" || codeParts[i] == "b") continue;
                    if (i >= maxMessageLength) break;

                    if (i > 0 && !FragmentMatchesAny(code.Replace(" ", "").Substring(0, i)))
                    {
                        checkMessages = false;
                        break;
                    };

                    checkMessages = false;
                    string[] splitCode = code.Split(" ");
                    foreach (string opt in Rules[int.Parse(codeParts[i])].Split(" | "))
                    {
                        splitCode[i] = opt;
                        possibilities.Enqueue(string.Join(" ", splitCode));
                    }
                    break;
                }

                if (checkMessages)
                {
                    code = code.Replace(" ", "");
                    for (int i = 0; i < Messages.Count; i++)
                    {
                        if (Messages[i] == code)
                        {
                            matches++;
                            Messages.RemoveAt(i);
                            i--;
                        }
                    }
                }
            }

            return matches;
        }

        private bool FragmentMatchesAny(string codeFragment)
        {
            for (int i = 0; i < Messages.Count; i++)
            {
                if (Messages[i].Length >= codeFragment.Length && Messages[i].Substring(0, codeFragment.Length) == codeFragment) return true;
            }
            return false;
        }

        public override object Task2()
        {
            ParseRules();
            ParseMessages();

            Rules[8] = "42 | 42 8";
            Rules[11] = "42 31 | 42 11 31";

            return FindMatches(0);
        }
    }
}