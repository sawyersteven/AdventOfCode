using System.Collections.Generic;
using AdventOfCode;

namespace Advent2015
{
    public class Challenge16 : Challenge
    {
        private IEnumerable<Dictionary<string, int>> ParseInput()
        {
            char[] splt = new char[] { ':', ' ', ',' };
            foreach (string line in input)
            {
                Dictionary<string, int> sue = new Dictionary<string, int>();
                string[] parts = line.Split(splt, System.StringSplitOptions.RemoveEmptyEntries);
                for (int i = 2; i < parts.Length; i += 2)
                {
                    sue.Add(parts[i], int.Parse(parts[i + 1]));
                }
                yield return sue;
            }
        }

        Dictionary<string, int> forensicReport = new Dictionary<string, int>()
            {{"children", 3},
             {"cats", 7},
             {"samoyeds", 2},
             {"pomeranians", 3},
             {"akitas", 0},
             {"vizslas", 0},
             {"goldfish", 5},
             {"trees", 3},
             {"cars", 2},
             {"perfumes", 1}};

        public override object Task1()
        {
            int i = 1;
            foreach (var sue in ParseInput())
            {
                bool ok = true;
                foreach (var kv in sue)
                {
                    if (kv.Key == "ID") continue;
                    if (forensicReport[kv.Key] != kv.Value)
                    {
                        ok = false;
                        break;
                    }
                }
                if (ok) return i;
                i++;
            }
            return null;
        }

        public override object Task2()
        {
            int i = 1;
            foreach (var sue in ParseInput())
            {
                bool ok = true;
                foreach (var kv in sue)
                {
                    if (kv.Key == "ID") continue;
                    if (kv.Key == "trees" || kv.Key == "cats")
                    {
                        if (forensicReport[kv.Key] > kv.Value) ok = false;

                    }
                    else if (kv.Key == "pomeranians" || kv.Key == "goldfish")
                    {
                        if (forensicReport[kv.Key] < kv.Value) ok = false;
                    }

                    else if (forensicReport[kv.Key] != kv.Value) ok = false;

                    if (!ok) break;
                }
                if (ok) return i;
                i++;
            }
            return null;
        }
    }
}
