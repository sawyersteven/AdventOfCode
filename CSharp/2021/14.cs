using System;
using System.Collections.Generic;
using System.Linq;
using AdventOfCode;

namespace Advent2021
{
    public class Challenge14 : Challenge
    {
        private Dictionary<string, string> subs;
        private string template;

        public override void ParseInput()
        {
            subs = new Dictionary<string, string>();
            template = input[0];

            for (int i = 2; i < input.Length; i++)
            {
                string[] parts = input[i].Split(" -> ");
                string v = $"{parts[0][0]}{parts[1]}{parts[0][1]}";
                subs[parts[0]] = v;
            }
        }

        public override object Task1()
        {
            // gonna brute force this one...
            string polymer = template;
            for (int turn = 0; turn < 10; turn++)
            {
                string[] subbed = new string[polymer.Length];
                subbed[^1] = polymer[^1].ToString();
                for (int i = 0; i < polymer.Length - 1; i++)
                {
                    string key = polymer.Substring(i, 2);
                    subbed[i] = subs[key].Substring(0, 2);
                }
                polymer = string.Join(null, subbed);
            }
            return CountPolymer(polymer);
        }

        private int CountPolymer(string poly)
        {
            DefaultDict<char, int> count = new DefaultDict<char, int>();
            foreach (char c in poly)
            {
                count[c]++;
            }

            int min = int.MaxValue;
            int max = 0;

            foreach ((char c, int i) in count)
            {
                min = Math.Min(min, i);
                max = Math.Max(max, i);
            }

            return max - min;
        }

        public override object Task2()
        {
            // Everyone gets a dictionary!
            DefaultDict<string, long> pairCounts = new DefaultDict<string, long>();
            DefaultDict<string, long> tmp = new DefaultDict<string, long>();
            DefaultDict<char, long> charCounts = new DefaultDict<char, long>();

            for (int i = 0; i < template.Length; i++)
            {
                charCounts[template[i]]++;
            }

            for (int i = 0; i < template.Length - 1; i++)
            {
                charCounts[template[i]]++;
                pairCounts[template.Substring(i, 2)]++;
            }

            for (int _ = 0; _ < 40; _++)
            {
                foreach ((string k, long v) in pairCounts)
                {
                    if (v == 0) continue;
                    pairCounts[k] -= v;
                    string repl = subs[k];
                    tmp[repl.Substring(0, 2)] += v;
                    tmp[repl.Substring(1, 2)] += v;
                    charCounts[repl[1]] += v;
                }
                foreach ((string k, long v) in tmp)
                {
                    pairCounts[k] += v;
                    tmp[k] = 0;
                }
            }

            // I know I'm typically opposed to Linq but I'm tired of thinking right now
            return charCounts.Values.Max() - charCounts.Values.Min();
        }
    }
}
