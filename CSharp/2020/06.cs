using AdventOfCode;
using System.Collections.Generic;
using System.Linq;

namespace Advent2020
{
    public class Challenge06 : Challenge
    {

        public override object Task1()
        {
            int total = 0;
            HashSet<char> chars = new HashSet<char>();

            foreach (string line in input)
            {
                if (line == string.Empty)
                {
                    total += chars.Count;
                    chars.Clear();
                    continue;
                }
                foreach (char c in line) chars.Add(c);
            }

            total += chars.Count;

            return total;
        }


        private char[] az = new char[26];
        public override object Task2()
        {
            for (char i = (char)0; i < az.Length; i++)
            {
                az[i] = (char)(i + 'a');
            }

            int total = 0;

            List<string> group = new List<string>();

            foreach (string line in input)
            {
                if (line == string.Empty)
                {
                    total += IntersectStrings(group).Length;
                    group.Clear();
                    continue;
                }
                group.Add(line);
            }

            total += IntersectStrings(group).Length;
            return total;
        }

        private string IntersectStrings(List<string> group)
        {
            SortedSet<char> c = new SortedSet<char>(group[0]);
            for (int i = 1; i < group.Count(); i++)
            {
                c.IntersectWith(group[i]);
            }
            return string.Join("", c);
        }
    }
}
