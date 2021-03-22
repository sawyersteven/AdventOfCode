using AdventOfCode;
using System.Collections.Generic;

namespace Advent2020
{
    public class Challenge07 : Challenge
    {
        private Dictionary<string, (string, int)[]> parsedInput;

        private void ParseInput()
        {
            Dictionary<string, (string, int)[]> bagTypes = new Dictionary<string, (string, int)[]>();

            foreach (string line in input)
            {
                string[] parts = line.Split(" bags", 2);
                string parent = parts[0];
                parts[1] = parts[1].Substring(8);
                if (parts[1] == " no other bags.")
                {
                    bagTypes.Add(parent, new (string, int)[0]);
                    continue;
                }
                parts = parts[1].Split(",");
                (string, int)[] children = new (string, int)[parts.Length];
                for (int i = 0; i < parts.Length; i++)
                {
                    children[i].Item1 = parts[i].Substring(3).Split(" bag")[0];
                    children[i].Item2 = int.Parse(parts[i].Substring(0, 3));
                }
                bagTypes.Add(parent, children);
            }
            parsedInput = bagTypes;
        }

        public override object Task1()
        {
            ParseInput();
            return ParentBags("shiny gold").Count;
        }

        private HashSet<string> ParentBags(string childBag)
        {
            HashSet<string> parents = new HashSet<string>();
            foreach (KeyValuePair<string, (string, int)[]> kv in parsedInput)
            {
                foreach ((string, int) v in kv.Value)
                {
                    if (v.Item1 == childBag)
                    {
                        foreach (string i in ParentBags(kv.Key))
                        {
                            parents.Add(i);
                        }
                        parents.Add(kv.Key);
                        break;
                    }
                }
            }
            return parents;
        }

        private int CountChildren(string bagName)
        {
            int count = 0;

            foreach ((string, int) children in parsedInput[bagName])
            {
                count += children.Item2;
                count += CountChildren(children.Item1) * children.Item2;
            }
            return count;
        }

        public override object Task2()
        {
            return CountChildren("shiny gold");
        }
    }
}