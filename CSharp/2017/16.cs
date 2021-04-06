using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge16 : Challenge
    {
        public override object Task1()
        {
            CircularLinkedList<char> programs = new CircularLinkedList<char>();
            for (char i = 'a'; i <= 'p'; i++) programs.AddLast(i);

            ApplyDance(programs);
            return string.Join("", programs);
        }

        private void ApplyDance(CircularLinkedList<char> programs)
        {
            foreach (string instruction in rawInput.Split(','))
            {
                switch (instruction[0])
                {
                    case 's':
                        programs.MoveHeadLeft(int.Parse(instruction.Substring(1)));
                        break;
                    case 'x':
                        int[] indexes = Array.ConvertAll(instruction.Substring(1).Split('/'), int.Parse);
                        var Ax = programs.ElementAt(indexes[0]);
                        var Bx = programs.ElementAt(indexes[1]);
                        var tmp = Ax.Value;
                        Ax.Value = Bx.Value;
                        Bx.Value = tmp;
                        break;
                    case 'p':
                        var Ap = programs.FindElementByValue(instruction[1]);
                        var Bp = programs.FindElementByValue(instruction[3]);
                        Ap.Value = instruction[3];
                        Bp.Value = instruction[1];
                        break;
                }
            }
        }

        public override object Task2()
        {
            CircularLinkedList<char> programs = new CircularLinkedList<char>();
            for (char i = 'a'; i <= 'p'; i++) programs.AddLast(i);

            HashSet<string> history = new HashSet<string>();
            int patternLen = 0; // this is only 24 with my input. Didn't expect it to be so low
            for (; ; patternLen++)
            {
                ApplyDance(programs);
                string id = string.Join("", programs);
                if (history.Contains(id)) break;
                history.Add(string.Join("", programs));
            }

            List<string> h = new List<string>(history);
            return h[(1_000_000_000 - 1) % patternLen];
        }
    }
}
