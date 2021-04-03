using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge07 : Challenge
    {
        string rootName;
        public override object Task1()
        {
            HashSet<string> parents = new HashSet<string>();
            HashSet<string> children = new HashSet<string>();
            foreach (string line in input)
            {
                parents.Add(line.Split(' ')[0]);
                if (line.Contains(">"))
                {
                    foreach (string child in line.Split("> ")[1].Split(", "))
                    {
                        children.Add(child);
                    }
                }
            }
            parents.ExceptWith(children);
            return rootName = new List<string>(parents)[0];
        }

        private class Prog
        {
            public Prog Parent;
            public string Name;
            public List<Prog> Children = new List<Prog>();
            public int Weight;
            private int tw = -1;
            public int TotalWeight
            {
                get
                {
                    if (tw == -1)
                    {
                        tw += Weight;
                        foreach (Prog c in Children)
                        {
                            tw += c.TotalWeight;
                        }
                    }
                    return tw;
                }
            }
        }

        public override object Task2()
        {
            Prog root = FillTree(rootName, null);

            Prog badProg = FindBadWeight(root);

            foreach (Prog p in badProg.Parent.Children)
            {
                if (p == badProg) continue;
                int weightDelta = badProg.TotalWeight - p.TotalWeight;
                return badProg.Weight - weightDelta;
            }
            return null;
        }

        /* Recrursively searches for the last prog node in a branch
        where the total weight doesn't match the total weight of its
        sibling nodes.
        */
        private Prog FindBadWeight(Prog root)
        {
            if (root.Children.Count == 0) return root;
            root.Children.Sort((a, b) => a.TotalWeight - b.TotalWeight);

            Prog badChild = null;
            if (root.Children[0].TotalWeight != root.Children[1].TotalWeight)
            {
                badChild = root.Children[0];
            }
            else if (root.Children[^1].TotalWeight != root.Children[^2].TotalWeight)
            {
                badChild = root.Children[^1];
            }

            if (badChild == null)
            {
                return root;
            }

            return FindBadWeight(badChild);
        }

        private Prog FillTree(string rootName, Prog parent)
        {
            Prog p = new Prog();
            p.Parent = parent;
            p.Name = rootName;
            foreach (string line in input)
            {
                if (line.StartsWith(rootName))
                {
                    p.Weight = int.Parse(line.Split('(')[1].Split(')')[0]);

                    if (line.Contains('>'))
                    {
                        foreach (string childName in line.Split("> ")[1].Split(", "))
                        {
                            p.Children.Add(FillTree(childName, p));
                        }
                    }
                    break;
                }
            }
            return p;
        }
    }
}
