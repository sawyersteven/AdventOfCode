using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge12 : Challenge
    {
        private Dictionary<string, List<string>> nodes = new Dictionary<string, List<string>>();
        public override void ParseInput()
        {
            foreach (string line in input)
            {
                string[] io = line.Split('-');
                if (!nodes.ContainsKey(io[0])) nodes[io[0]] = new List<string>();
                if (!nodes.ContainsKey(io[1])) nodes[io[1]] = new List<string>();

                if (io[1] != "start") nodes[io[0]].Add(io[1]);
                if (io[0] != "start") nodes[io[1]].Add(io[0]);
            }
        }

        public override object Task1()
        {
            Queue<string[]> q = new Queue<string[]>();
            q.Enqueue(new string[] { "start" });

            int count = 0;
            while (q.Count > 0)
            {
                string[] currentPath = q.Dequeue();
                string currentNode = currentPath[^1];

                foreach (string next in nodes[currentNode])
                {
                    if (next == "end")
                    {
                        count++;
                        continue;
                    }
                    if (next[0] > 96 && Array.IndexOf(currentPath, next) != -1) continue;
                    string[] n = new string[currentPath.Length + 1];
                    Array.Copy(currentPath, n, currentPath.Length);
                    n[^1] = next;
                    q.Enqueue(n);
                }
            }
            return count;
        }

        public override object Task2()
        {
            Queue<(string[], bool)> q = new Queue<(string[], bool)>();
            q.Enqueue((new string[] { "start" }, false));
            int count = 0;
            while (q.Count > 0)
            {
                (string[] currentPath, bool doubleSmallUsed) = q.Dequeue();
                string currentNode = currentPath[^1];

                foreach (string next in nodes[currentNode])
                {
                    if (next == "end")
                    {
                        count++;
                        continue;
                    }

                    bool alreadySeenSmall = next[0] > 96 && Array.IndexOf(currentPath, next) != -1;

                    if (alreadySeenSmall && doubleSmallUsed) continue;
                    bool dsu = doubleSmallUsed ^ alreadySeenSmall;

                    string[] n = new string[currentPath.Length + 1];
                    Array.Copy(currentPath, n, currentPath.Length);
                    n[^1] = next;
                    q.Enqueue((n, dsu));

                }
            }
            return count;
        }
    }
}