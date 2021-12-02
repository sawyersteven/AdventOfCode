using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge19 : Challenge
    {
        private string baseMolecule;
        private List<(string, string)> Replacements;

        public override void ParseInput()
        {
            Replacements = new List<(string, string)>();
            foreach (string line in input)
            {
                if (line == string.Empty) break;
                string[] parts = line.Split(" => ");
                Replacements.Add((parts[0], parts[1]));
            }
            baseMolecule = input[^1];
        }

        public override object Task1()
        {
            HashSet<string> combos = new HashSet<string>();

            foreach ((string a, string b) in Replacements)
            {
                foreach (int index in AllIndexesOf(baseMolecule, a))
                {
                    string replaced = baseMolecule.Remove(index, a.Length).Insert(index, b);
                    combos.Add(replaced);
                }
            }
            return combos.Count;
        }

        private IEnumerable<int> AllIndexesOf(string src, string substr)
        {
            int index = 0;
            for (; ; index += substr.Length)
            {
                index = src.IndexOf(substr, index);
                if (index == -1) break;
                yield return index;
            }

        }

        // I'm pretty sure this won't work on any possible molecule
        public override object Task2()
        {
            string current = baseMolecule;
            int count = 0;
            while (current != "e")
            {
                foreach ((string a, string b) in Replacements)
                {
                    int index = current.IndexOf(b);
                    if (index == -1) continue;
                    current = current.Remove(index, b.Length).Insert(index, a);
                    count++;
                }
            }
            return count;
        }
    }
}
