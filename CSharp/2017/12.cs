using System;
using System.Collections.Generic;
using AdventOfCode;

namespace Advent2017
{
    public class Challenge12 : Challenge
    {
        Dictionary<string, string[]> programs;
        public override object Task1()
        {
            programs = new Dictionary<string, string[]>();

            foreach (string line in input)
            {
                programs[line.Split(' ')[0]] = line.Split("> ")[1].Split(", ");
            }

            return ProgramGroup("0").Count;
        }

        private HashSet<string> ProgramGroup(string seed)
        {
            HashSet<string> goodProgs = new HashSet<string>() { seed };
            while (true)
            {
                int c = goodProgs.Count;
                foreach (var kv in programs)
                {
                    if (goodProgs.Contains(kv.Key))
                    {
                        goodProgs.UnionWith(kv.Value);
                    }
                }
                if (goodProgs.Count == c) break;
            }
            return goodProgs;
        }

        public override object Task2()
        {
            int groupCount = 0;
            HashSet<string> consumed = new HashSet<string>();

            foreach (string k in programs.Keys)
            {
                if (consumed.Contains(k)) continue;
                groupCount++;
                consumed.UnionWith(ProgramGroup(k));
            }

            return groupCount;
        }
    }
}
