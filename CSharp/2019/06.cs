using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2019
{
    public class Challenge06 : Challenge
    {
        Dictionary<string, List<string>> orbits;

        public override object Task1()
        {
            orbits = new Dictionary<string, List<string>>();
            foreach (string line in input)
            {
                string[] bodies = line.Split(')', 2);
                if (!orbits.ContainsKey(bodies[0]))
                {
                    orbits[bodies[0]] = new List<string>();
                }
                orbits[bodies[0]].Add(bodies[1]);
            }
            return CountOrbits("COM", 0);
        }

        private int CountOrbits(string hubBody, int depth)
        {
            if (orbits.ContainsKey(hubBody))
            {
                int count = depth;
                foreach (string child in orbits[hubBody])
                {
                    count += CountOrbits(child, depth + 1);
                }
                return count;
            }
            else
            {
                return depth;
            }
        }

        public override object Task2()
        {
            List<string> branches = FindBranches("COM");

            string[] branchA = branches[0].Split(',');
            Array.Reverse(branchA);
            string[] branchB = branches[1].Split(',');
            Array.Reverse(branchB);

            for (int i = 0; i < branchA.Length; i++)
            {
                int indB = Array.IndexOf(branchB, branchA[i]);
                if (indB != -1)
                {
                    return i + indB - 2;
                }
            }
            return null;
        }

        private List<string> FindBranches(string startFrom)
        {
            List<string> branches = new List<string>();

            if (!orbits.ContainsKey(startFrom))
            {
                if (startFrom == "YOU" || startFrom == "SAN")
                {
                    branches.Add(startFrom);
                }
                return branches;
            }

            foreach (string child in orbits[startFrom])
            {
                foreach (string c in FindBranches(child))
                {
                    branches.Add($"{startFrom},{c}");
                }
            }
            return branches;
        }

        private string FindParent(string body)
        {
            foreach (var kv in orbits)
            {
                if (kv.Value.Contains(body))
                {
                    return kv.Key;
                }
            }
            return null;
        }

        private int FindSAN(string currentBody, int currentJumps)
        {
            if (orbits[currentBody].Contains("SAN"))
            {
                return currentJumps;
            }

            string parent = FindParent(currentBody);

            foreach (string child in orbits[parent])
            {
                if (child == currentBody || !orbits.ContainsKey(child)) continue;
                int steps = FindSAN(child, currentJumps + 1);
                if (steps == -1) continue;
                return steps;
            }
            return FindSAN(parent, currentJumps + 1); ;
        }
    }
}
