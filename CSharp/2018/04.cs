using AdventOfCode;
using System;
using System.Collections.Generic;

namespace Advent2018
{
    public class Challenge04 : Challenge
    {
        Dictionary<string, int[]> guardNaps;
        public override object Task1()
        {
            Array.Sort(input);
            guardNaps = new Dictionary<string, int[]>();

            for (int i = 0; i < input.Length;)
            {
                string id = input[i].Split('#')[1].Split(' ')[0];
                if (!guardNaps.ContainsKey(id)) guardNaps[id] = new int[60];
                int[] napMap = guardNaps[id];
                i++;
                while (i < input.Length && !input[i].Contains('#'))
                {
                    int start = int.Parse(input[i].Split(':')[1].Substring(0, 2));
                    i++;
                    int end = int.Parse(input[i].Split(':')[1].Substring(0, 2));
                    i++;
                    for (int j = start; j < end; j++)
                    {
                        napMap[j]++;
                    }
                }
            }

            string sleepyGuard = "";
            int minutesSlept = 0;
            foreach (var kv in guardNaps)
            {
                int t = 0;
                for (int i = 0; i < kv.Value.Length; i++)
                {
                    t += kv.Value[i];
                }
                if (t > minutesSlept)
                {
                    sleepyGuard = kv.Key;
                    minutesSlept = t;
                }
            }

            int bestMinute = 0;
            int minuteValue = 0;
            for (int i = 0; i < guardNaps[sleepyGuard].Length; i++)
            {
                if (guardNaps[sleepyGuard][i] > minuteValue)
                {
                    minuteValue = guardNaps[sleepyGuard][i];
                    bestMinute = i;
                }
            }

            return int.Parse(sleepyGuard) * bestMinute;
        }

        public override object Task2()
        {

            string guardID = "";
            int bestMinute = 0;
            int bestMinVal = 0;

            foreach (var kv in guardNaps)
            {
                for (int i = 0; i < kv.Value.Length; i++)
                {
                    if (kv.Value[i] > bestMinVal)
                    {
                        bestMinVal = kv.Value[i];
                        bestMinute = i;
                        guardID = kv.Key;
                    }
                }
            }

            return int.Parse(guardID) * bestMinute;
        }
    }
}